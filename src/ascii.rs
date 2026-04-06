use crate::*;

/// A simple renderer for displaying ASCII text. Useful for quick debugging and displaying simple information.
///
/// Example:
/// ```ignore
/// use pgfx::ascii::{AsciiRender, AsciiText};
///
/// // Initialization step:
/// let ascii_shader = ProgramRef::new("ascii_shader", include_str!("ascii_shader.txt"));
/// let ascii_render = AsciiRender::new(ascii_shader.with_vertex_entry("main_vs"), ascii_shader.with_fragment_entry("main_fs"));
///
/// // Usage (within a render pass):
/// let mut pass = ...;
/// let surface_dim = window.logical_size();
///
/// // Render some text in the top-left corner
/// ascii_render.render_top(&mut pass, surface_dim, "Hello, world!")?;
///
/// // Render some text showcasing more advanced settings
/// let mut txt = AsciiText::new();
/// txt.set_placement(AsciiHorz::Center, AsciiVert::Center);
/// txt.set_horz_alignment(AsciiHorz::Center);
/// txt.set_line_font_size(48.);
/// txt.push_str("The\nTitle");
/// txt.start_line();
/// txt.set_line_font_size(32.);
/// txt.set_color([0.75, 0.75, 0.75, 1.0].into());
/// txt.push_str("Subtitle");
/// ascii_render.render(&mut pass, surface_dim, &txt)?;
///
/// // Render a table of values
/// let mut txt = AsciiText::new();
/// txt.set_tab_width(10);
/// txt.push_str("Name\tValue\n");
/// txt.push_str("----\t-----\n");
/// txt.push_str("FPS\t60\n");
/// txt.push_str("Sim Time\t1.29\n");
/// txt.push_str("Scene\tMain Menu\n");
/// ascii_render.render(&mut pass, surface_dim, &txt)?;
/// ```
pub struct AsciiRender<B: Backend> {
    pipeline: Pipeline<B>,
    vertex_shader: Program,
    fragment_shader: Program,
}
impl<B: Backend> AsciiRender<B> {
    /// Creates a new ASCII renderer with the provided vertex and fragment shader.
    ///
    /// The vertex shader will recieve the vertex attributes `pos: vec2`, `uv: vec2`, and `color: vec4`. The position range will
    /// be [-1, -1] (lower-left) to [1, 1] (upper-right). Additionally, the texture (single UNorm8 channel) and shader will be
    /// bound to the pipeline.
    ///
    /// All values follow normal OpenGL conventions, so the top-left corner of the texture is (0, 0) and the bottom-right is (1, 1).
    pub fn new(vertex_shader: impl Into<Program>, fragment_shader: impl Into<Program>) -> Self {
        let pipeline = Pipeline::new("ascii");

        Self {
            pipeline,
            vertex_shader: vertex_shader.into(),
            fragment_shader: fragment_shader.into(),
        }
    }

    /// Renders the provided ASCII text. The provided surface dimensions are used to calculate the position of the text based on
    /// the placement and offset settings.
    pub fn render(
        &self,
        pass: &mut RenderPass<'_, B>,
        surface_dim: [f32; 2],
        text: &AsciiText,
    ) -> Result<(), B::Error> {
        self.render_all(pass, surface_dim, std::iter::once(text))
    }

    pub fn render_all<'a>(
        &self,
        pass: &mut RenderPass<'_, B>,
        surface_dim: [f32; 2],
        text: impl Iterator<Item = &'a AsciiText>,
    ) -> Result<(), B::Error> {
        // Create the vertex buffer
        let mut vbuf = Vec::with_capacity(512);
        text.for_each(|t| t.render(surface_dim, &mut vbuf));

        if vbuf.is_empty() {
            // Nothing to render, so skip the rest of the work
            return Ok(());
        }

        // Create the index buffer. Each character is 4 vertexes and 6 indexes.
        let num_chars = vbuf.len() / 4;
        let mut ibuf = Vec::with_capacity(num_chars * 6);
        for i in 0..num_chars as u32 {
            let idx = i * 4;
            ibuf.push([idx + 0, idx + 1, idx + 2]);
            ibuf.push([idx + 2, idx + 3, idx + 0]);
        }

        pass.run(&self.pipeline)
            .load_array_input(&vbuf)
            .load_array_input(&ibuf)
            .execute(|cfg| {
                cfg.load_input(&self.vertex_shader)?;
                cfg.load_input(&self.fragment_shader)?;

                cfg.load_input(&create_ascii_texture())?;
                cfg.load_input(&Sampler::nearest_clamp())?;

                cfg.alpha_blending(true);

                Ok(())
            })
    }

    /// A convenience method for rendering white text placed in the top-left corner.
    pub fn render_top(
        &self,
        pass: &mut RenderPass<'_, B>,
        surface_dim: [f32; 2],
        s: &str,
    ) -> Result<(), B::Error> {
        let mut ascii_text = AsciiText::new();
        ascii_text.set_placement(AsciiHorz::Left, AsciiVert::Top);
        ascii_text.set_color(LrgbaF32::new(1.0, 1.0, 1.0, 1.0));
        ascii_text.set_offset([4.0, 4.0]);
        ascii_text.push_str(s);

        self.render(pass, surface_dim, &ascii_text)
    }
    /// A convenience method for rendering white text placed in the bottom-left corner.
    pub fn render_bottom(
        &self,
        pass: &mut RenderPass<'_, B>,
        surface_dim: [f32; 2],
        s: &str,
    ) -> Result<(), B::Error> {
        let mut ascii_text = AsciiText::new();
        ascii_text.set_placement(AsciiHorz::Left, AsciiVert::Bottom);
        ascii_text.set_color(LrgbaF32::new(1.0, 1.0, 1.0, 1.0));
        ascii_text.set_offset([4.0, 4.0]);
        ascii_text.push_str(s);

        self.render(pass, surface_dim, &ascii_text)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AsciiHorz {
    Left,
    Center,
    Right,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AsciiVert {
    Top,
    Center,
    Bottom,
}

/// A struct representing a block of ASCII text to be rendered.
#[derive(Debug, Clone)]
pub struct AsciiText {
    // Placement
    horz_placement: AsciiHorz,
    vert_placement: AsciiVert,
    offset: [f32; 2],
    font_dim: [f32; 2],
    tab_width: u32,

    // ASCII content
    cur_x: u32,
    cur_horz_alignment: AsciiHorz,
    cur_color: LrgbaF32,
    lines: Vec<AsciiLine>,
}
impl AsciiText {
    /// Creates a new empty AsciiText with default settings. The default settings are:
    /// - Placement: Top-left
    /// - Offset: [0, 0]
    /// - Font size: 24
    /// - Horizontal alignment: Left
    /// - Color: White
    /// - Tab width: aligned to 4 spaces
    pub fn new() -> Self {
        let font_size = 24.0;
        let font_dim = [font_size * 0.5, font_size];
        Self {
            horz_placement: AsciiHorz::Left,
            vert_placement: AsciiVert::Top,
            offset: [0.0, 0.0],
            font_dim,
            tab_width: 4,
            cur_x: 0,
            cur_horz_alignment: AsciiHorz::Left,
            cur_color: LrgbaF32::new(1.0, 1.0, 1.0, 1.0),
            lines: vec![AsciiLine::new(AsciiHorz::Left, font_dim)],
        }
    }
    /// Creates an AsciiText and initializes it with the provided function. Useful for creating and initializing in one
    /// expression, such as when passing to a render function.
    ///
    /// Example:
    /// ```
    /// use pgfx::ascii::AsciiText;
    ///
    /// let ascii_text = AsciiText::create(|t| t.set_color([1.0, 0.0, 0.0, 1.0].into()).push_str("Red text"));
    /// ```
    pub fn create(init: impl for<'a> FnOnce(&'a mut Self) -> &'a mut Self) -> Self {
        let mut v = Self::new();
        init(&mut v);
        v
    }

    /// Sets the font size for the current line.
    ///
    /// Note that this applies to the entire line.
    pub fn set_line_font_size(&mut self, size: f32) -> &mut Self {
        self.font_dim = [size * 0.5, size];
        self.lines.last_mut().unwrap().font_dim = self.font_dim;
        self
    }

    /// Sets the font dimensions (width, height) for the current line.
    ///
    /// Note that this applies to the entire line.
    pub fn set_line_font_dim(&mut self, dim: [f32; 2]) -> &mut Self {
        self.font_dim = dim;
        self.lines.last_mut().unwrap().font_dim = self.font_dim;
        self
    }

    /// Defines the starting placement location for the text. This is the point that the text will be placed relative to.
    pub fn set_placement(&mut self, horz: AsciiHorz, vert: AsciiVert) -> &mut Self {
        self.horz_placement = horz;
        self.vert_placement = vert;
        self
    }

    pub fn set_placement_top_left(&mut self) -> &mut Self {
        self.set_placement(AsciiHorz::Left, AsciiVert::Top)
    }
    pub fn set_placement_top_center(&mut self) -> &mut Self {
        self.set_placement(AsciiHorz::Center, AsciiVert::Top)
    }
    pub fn set_placement_top_right(&mut self) -> &mut Self {
        self.set_placement(AsciiHorz::Right, AsciiVert::Top)
    }
    pub fn set_placement_center_left(&mut self) -> &mut Self {
        self.set_placement(AsciiHorz::Left, AsciiVert::Center)
    }
    pub fn set_placement_center(&mut self) -> &mut Self {
        self.set_placement(AsciiHorz::Center, AsciiVert::Center)
    }
    pub fn set_placement_center_right(&mut self) -> &mut Self {
        self.set_placement(AsciiHorz::Right, AsciiVert::Center)
    }
    pub fn set_placement_bottom_left(&mut self) -> &mut Self {
        self.set_placement(AsciiHorz::Left, AsciiVert::Bottom)
    }
    pub fn set_placement_bottom_center(&mut self) -> &mut Self {
        self.set_placement(AsciiHorz::Center, AsciiVert::Bottom)
    }
    pub fn set_placement_bottom_right(&mut self) -> &mut Self {
        self.set_placement(AsciiHorz::Right, AsciiVert::Bottom)
    }

    /// Sets the offset from the placement location.
    ///
    /// For example, if the placement is the top left and the offset is `[0, 10]`, then the text will be placed 10 pixels below
    /// the top-left corner of the surface.
    pub fn set_offset(&mut self, offset: [f32; 2]) -> &mut Self {
        self.offset = offset;
        self
    }

    /// Sets the horizontal alignment for the current line and all future lines. This does not affect previous lines.
    pub fn set_horz_alignment(&mut self, horz_alignment: AsciiHorz) -> &mut Self {
        self.cur_horz_alignment = horz_alignment;
        self.lines.last_mut().unwrap().horz_alignment = horz_alignment;
        self
    }

    /// Sets the font color, which will be used for all future characters until changed. This does not affect previous characters.
    pub fn set_color(&mut self, color: LrgbaF32) -> &mut Self {
        self.cur_color = color;
        self
    }

    /// Sets the tab alignment width in spaces. For example, if the tab width is 4, then tabs will be aligned to multiples
    /// of 4 spaces (0, 4, 8, 12, etc).
    pub fn set_tab_width(&mut self, width: u32) -> &mut Self {
        self.tab_width = width;
        self
    }

    /// Starts a new line if the current line is not empty.
    pub fn start_line(&mut self) -> &mut Self {
        if !self.lines.last().unwrap().is_empty() {
            self.lines
                .push(AsciiLine::new(self.cur_horz_alignment, self.font_dim));
            self.cur_x = 0;
        }
        self
    }

    /// Adds the provided text.
    pub fn push_str(&mut self, txt: &str) -> &mut Self {
        self.push_ascii(txt.as_bytes());
        self
    }

    /// Adds the provided ASCII text.
    pub fn push_ascii(&mut self, txt: &[u8]) -> &mut Self {
        for &b in txt {
            if b == b'\n' {
                // Start a new line
                self.lines
                    .push(AsciiLine::new(self.cur_horz_alignment, self.font_dim));
                self.cur_x = 0;
            } else if b == b'\t' {
                let line = self.lines.last_mut().unwrap();

                // Tab is aligned to the tab width
                let spaces = self.tab_width - (self.cur_x % self.tab_width);
                self.cur_x += spaces;
                line.width = self.cur_x;
            } else {
                let line = self.lines.last_mut().unwrap();

                if b != b' ' {
                    // Add the character to the line
                    line.chars.push(AsciiChar {
                        x: self.cur_x,
                        color: self.cur_color,
                        glyph: b,
                    });
                }

                self.cur_x += 1;
                line.width = self.cur_x;
            }
        }
        self
    }

    fn render(&self, win_dim: [f32; 2], vbuf: &mut Vec<AsciiVertex>) {
        let lines = if self.lines.last().unwrap().is_empty() {
            &self.lines[..self.lines.len() - 1]
        } else {
            &self.lines
        };

        let mut box_dim = [0.0, 0.0f32];
        for line in lines {
            let dim = line.dim();
            box_dim[0] = box_dim[0].max(dim[0]);
            box_dim[1] += dim[1];
        }

        let x0 = match self.horz_placement {
            AsciiHorz::Left => self.offset[0],
            AsciiHorz::Center => (win_dim[0] - box_dim[0]) * 0.5 + self.offset[0],
            AsciiHorz::Right => win_dim[0] - box_dim[0] - self.offset[0],
        };
        let mut y0 = match self.vert_placement {
            AsciiVert::Top => self.offset[1],
            AsciiVert::Center => (win_dim[1] - box_dim[1]) * 0.5 + self.offset[1],
            AsciiVert::Bottom => win_dim[1] - box_dim[1] - self.offset[1],
        };

        for line in self.lines.iter() {
            line.render(x0, y0, box_dim[0], vbuf);
            y0 += line.font_dim[1];
        }

        // Re-scale: The shader expects the position to be in the range [-1, -1] to [1, 1] using OpenGL conventions
        let scale = [2.0 / win_dim[0], 2.0 / win_dim[1]];
        for v in vbuf.iter_mut() {
            v.pos[0] = v.pos[0] * scale[0] - 1.0;
            v.pos[1] = -(v.pos[1] * scale[1] - 1.0);
        }
    }
}

#[derive(Debug, Clone)]
struct AsciiLine {
    chars: Vec<AsciiChar>,
    width: u32,
    horz_alignment: AsciiHorz,
    font_dim: [f32; 2],
}
impl AsciiLine {
    fn new(horz_alignment: AsciiHorz, font_dim: [f32; 2]) -> Self {
        Self {
            chars: Vec::new(),
            width: 0,
            horz_alignment,
            font_dim,
        }
    }
    fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }

    fn dim(&self) -> [f32; 2] {
        [self.width as f32 * self.font_dim[0], self.font_dim[1]]
    }

    fn render(&self, x0: f32, y0: f32, box_width: f32, vbuf: &mut Vec<AsciiVertex>) {
        let [fw, fh] = self.font_dim;

        let line_width = self.width as f32 * fw;

        let x0 = match self.horz_alignment {
            AsciiHorz::Left => x0,
            AsciiHorz::Center => x0 + (box_width - line_width) * 0.5,
            AsciiHorz::Right => x0 + box_width - line_width,
        };

        const UV_DIM: f32 = 1.0 / 16.0; // 16x16 grid of characters in the texture

        for c in &self.chars {
            let x = x0 + c.x as f32 * fw;
            let y = y0;

            let u = (c.glyph % 16) as f32 * UV_DIM;
            let v = (c.glyph / 16) as f32 * UV_DIM;

            let color = c.color.to_array();

            vbuf.push(AsciiVertex {
                pos: [x, y],
                uv: [u, v],
                color,
            });
            vbuf.push(AsciiVertex {
                pos: [x + fw, y],
                uv: [u + UV_DIM, v],
                color,
            });
            vbuf.push(AsciiVertex {
                pos: [x + fw, y + fh],
                uv: [u + UV_DIM, v + UV_DIM],
                color,
            });
            vbuf.push(AsciiVertex {
                pos: [x, y + fh],
                uv: [u, v + UV_DIM],
                color,
            });
        }
    }
}

#[derive(Debug, Clone)]
struct AsciiChar {
    x: u32,
    color: LrgbaF32,
    glyph: u8,
}

vertex_struct!(
    struct AsciiVertex {
        pub pos: [f32; 2],
        pub uv: [f32; 2],
        pub color: [f32; 4],
    }
);

fn create_ascii_texture() -> Texture<pixel::UNorm8> {
    let mut img = Texture::new([128, 128]);

    for y in 0..128u32 {
        for x in 0..128u32 {
            let v = if ASCII_IMG[y as usize][x as usize] == b' ' {
                0
            } else {
                255
            };
            img.set([x, y], pixel::UNorm8(v));
        }
    }

    img
}

const ASCII_IMG: [[u8; 128]; 128] = [
    *b"         ######  ######  ## ##     #      ###      #            ########        ########    ####  ####    ###### ########  ##  #",
    *b"        #      ################   ###    #####     #            ########  ####  ##    ##     ### ##  ##   ##  ## ##   ## # ## # ",
    *b"        # #  # ### ## #########  #####    ###     ###      ##   ###  ### ##  ## #  ##  #    #### ##  ##   ###### #######  ####  ",
    *b"        #      ################ ####### #######  #####    ####  ##    ## #    # # #### # ##### # ##  ##   ##     ##   #####  ###",
    *b"        # #### ###    ## #####   #####  ####### #######   ####  ##    ## #    # # #### ###  ##    ####    ##     ##   #####  ###",
    *b"        #  ##  ####  ###  ###     ###   ## # ##  #####     ##   ###  ### ##  ## #  ##  ###  ##     ##    ###     ##  ###  ####  ",
    *b"        #      #########   #       #       #       #            ########  ####  ##    ####  ##   ###### ####    ###  ##  # ## # ",
    *b"         ######  ######                   ###     ###           ########        ######## ####      ##   ###     ##      #  ##  #",
    *b"#             #    ##    ##  ##  ####### ######            ##      ##      ##                                                   ",
    *b"###         ###   ####   ##  ## ## ## ####    ##          ####    ####     ##      ##     ##              #  #     ##   ########",
    *b"#####     #####  ######  ##  ## ## ## ## ####            ######  ######    ##       ##   ##     ##       ##  ##   ####  ########",
    *b"####### #######    ##    ##  ##  #### ####  ##             ##      ##      ##   ####### ####### ##      ######## ######  ###### ",
    *b"#####     #####    ##    ##  ##    ## ####  ##   ######  ######    ##    ######     ##   ##     ##       ##  ## ########  ####  ",
    *b"###         ###  ######            ## ## ####    ######   ####     ##     ####     ##     ##    #######   #  #  ########   ##   ",
    *b"#             #   ####   ##  ##    ## ###   ##   ######    ##      ##      ##                                                   ",
    *b"                   ##                   #####           ########                                                                ",
    *b"          ##     ## ##   ## ##    ##              ###    ##        ##    ##                                                  ## ",
    *b"         ####    ## ##   ## ##   #####  ##   ##  ## ##   ##       ##      ##     ##  ##   ##                                ##  ",
    *b"         ####    ## ##  ####### ##      ##  ##    ###   ##       ##        ##     ####    ##                               ##   ",
    *b"          ##             ## ##   ####      ##    ### ##          ##        ##   ##############          ######            ##    ",
    *b"          ##            #######     ##    ##    ## ###           ##        ##     ####    ##                             ##     ",
    *b"                         ## ##  #####    ##  ## ##  ##            ##      ##     ##  ##   ##     ###              ##    ##      ",
    *b"          ##             ## ##    ##    ##   ##  ### ##            ##    ##                       ##              ##    #       ",
    *b"                                                                                                 ##                             ",
    *b" ####     ##     ####    ####      ###  ######    ###   ######   ####    ####                      ##            ##      ####   ",
    *b"##  ##  ####    ##  ##  ##  ##    ####  ##       ##     ##  ##  ##  ##  ##  ##                    ##              ##    ##  ##  ",
    *b"## ###    ##        ##      ##   ## ##  #####   ##          ##  ##  ##  ##  ##    ##      ##     ##     ######     ##       ##  ",
    *b"######    ##      ###     ###   ##  ##      ##  #####      ##    ####    #####    ##      ##    ##                  ##     ##   ",
    *b"### ##    ##     ##         ##  #######     ##  ##  ##    ##    ##  ##      ##                   ##     ######     ##     ##    ",
    *b"##  ##    ##    ##  ##  ##  ##      ##  ##  ##  ##  ##   ##     ##  ##     ##     ##     ###      ##              ##            ",
    *b" ####   ######  ######   ####       ##   ####    ####    ##      ####    ###      ##      ##       ##            ##       ##    ",
    *b"                                                                                         ##                                     ",
    *b" #####    ##    ######    ####  ######  ####### #######   ####  ##  ##   ####      #### ###  ## ####    ##   ## ##   ##   ###   ",
    *b"##   ##  ####    ##  ##  ##  ##  ## ##   ##   #  ##   #  ##  ## ##  ##    ##        ##   ##  ##  ##     ### ### ###  ##  ## ##  ",
    *b"## #### ##  ##   ##  ## ##       ##  ##  ## #    ## #   ##      ##  ##    ##        ##   ## ##   ##     ####### #### ## ##   ## ",
    *b"## #### ##  ##   #####  ##       ##  ##  ####    ####   ##      ######    ##        ##   ####    ##     ## # ## ## #### ##   ## ",
    *b"## #### ######   ##  ## ##       ##  ##  ## #    ## #   ##  ### ##  ##    ##    ##  ##   ## ##   ##   # ##   ## ##  ### ##   ## ",
    *b"##      ##  ##   ##  ##  ##  ##  ## ##   ##   #  ##      ##  ## ##  ##    ##    ##  ##   ##  ##  ##  ## ##   ## ##   ##  ## ##  ",
    *b" ####   ##  ##  ######    ####  ######  ####### ####      ##### ##  ##   ####    ####   ###  ## ####### ##   ## ##   ##   ###   ",
    *b"                                                                                                                                ",
    *b"######   ####   ######   ####   ######  ##  ##  ##  ##  ##   ## ##   ## ##  ##  #######  ####   ##       ####      #            ",
    *b" ##  ## ##  ##   ##  ## ##  ##  # ## #  ##  ##  ##  ##  ##   ## ##   ## ##  ##  ##  ##   ##      ##        ##     ###           ",
    *b" ##  ## ##  ##   ##  ## ###       ##    ##  ##  ##  ##  ##   ##  ## ##  ##  ##  #  ##    ##       ##       ##    ## ##          ",
    *b" #####  ##  ##   #####    ###     ##    ##  ##  ##  ##  ## # ##   ###    ####     ##     ##        ##      ##   ##   ##         ",
    *b" ##     ## ###   ####      ###    ##    ##  ##  ##  ##  #######  ## ##    ##     ##   #  ##         ##     ##                   ",
    *b" ##      ####    ## ##  ##  ##    ##    ##  ##   ####   ### ### ##   ##   ##    ##   ##  ##          ##    ##                   ",
    *b"####       ###  ###  ##  ####    ####   ######    ##    ##   ## ##   ##  ####   #######  ####         #  ####                   ",
    *b"                                                                                                                        ########",
    *b"  ##            ###                ###            ###           ###       ##       ##   ###      ###                            ",
    *b"  ##             ##                 ##           ## ##           ##                      ##       ##                            ",
    *b"   ##    ####    #####   ####       ##   ####    ##      ### ##  ## ##   ###     ####    ##  ##   ##    ### ##  #####    ####   ",
    *b"            ##   ##  ## ##  ##   #####  ##  ##  ####    ##  ##   ### ##   ##       ##    ## ##    ##    ####### ##  ##  ##  ##  ",
    *b"         #####   ##  ## ##      ##  ##  ######   ##     ##  ##   ##  ##   ##       ##    ####     ##    ## # ## ##  ##  ##  ##  ",
    *b"        ##  ##   ##  ## ##  ##  ##  ##  ##       ##      #####   ##  ##   ##       ##    ## ##    ##    ##   ## ##  ##  ##  ##  ",
    *b"         ### ## # ####   ####    ### ##  ####   ####        ##  ###  ##  ####   ## ##   ###  ##  ####   ##   ## ##  ##   ####   ",
    *b"                                                        #####                    ###                                            ",
    *b"                                   #                                                       ###     ##   ###      ### ##    #    ",
    *b"                                  ##                                                      ##       ##     ##    ## ###    ###   ",
    *b"## ###   ### ## ## ##    #####   #####  ##  ##  ##  ##  ##   ## ##   ## ##  ##  ######    ##       ##     ##             ## ##  ",
    *b" ##  ## ##  ##   ## ##  ##        ##    ##  ##  ##  ##  ##   ##  ## ##  ##  ##  #  ##   ###                ###          ##   ## ",
    *b" ##  ## ##  ##   ## ##   ####     ##    ##  ##  ##  ##  ## # ##   ###   ##  ##    ##      ##       ##     ##            ##   ## ",
    *b" #####   #####   ##         ##    ## #  ##  ##   ####   #######  ## ##   #####   ##  #    ##       ##     ##            ##   ## ",
    *b" ##         ##  ####    #####      ##    ### ##   ##     ## ##  ##   ##     ##  ######     ###     ##   ###             ####### ",
    *b"####       ####                                                         #####                                                   ",
    *b" ####              ###   ###### ##  ##  ###       ##             ###### ##  ##  ###     ##  ##   #####  ###     ##  ##    ##    ",
    *b"##  ##  ##  ##          ##    ##                  ##            ##    ##                        ##   ##           ##      ##    ",
    *b"##               ####     ####   ####    ####    ####    #####    ####   ####    ####    ###      ###    ###     ####           ",
    *b"##  ##  ##  ##  ##  ##       ##     ##      ##      ##  ##       ##  ## ##  ##  ##  ##    ##       ##     ##    ##  ##   ####   ",
    *b" ####   ##  ##  ######    #####  #####   #####   #####  ##       ###### ######  ######    ##       ##     ##    ##  ##  ##  ##  ",
    *b"   ##   ##  ##  ##       ##  ## ##  ##  ##  ##  ##  ##   #####   ##     ##      ##        ##       ##     ##    ######  ######  ",
    *b"    ##   ######  ####     ###### ######  ######  ######      ##   ####   ####    ####    ####     ####   ####   ##  ##  ##  ##  ",
    *b" ####                                                     ####                                                                  ",
    *b"   ###            #####  ####                    ####                   ##   ## ##  ##     ##     ###   ##  ##  ####        ### ",
    *b"                 ## ##  ##  ##  ##  ##  ###     ##  ##  ###     ##  ##    ###              ##    ## ##  ##  ##  ## ##      ## ##",
    *b"######   #########  ##                                                   #####  ##  ##   ######  ##  #   ####   ## ##      ##   ",
    *b" ##         ##  #######  ####    ####    ####   ##  ##  ##  ##  ##  ##  ##   ## ##  ##  ##      ####    ######  #### #   ###### ",
    *b" ####    #########  ##  ##  ##  ##  ##  ##  ##  ##  ##  ##  ##  ##  ##  ##   ## ##  ##  ##       ##       ##    ##  ##     ##   ",
    *b" ##     ##  ##  ##  ##  ##  ##  ##  ##  ##  ##  ##  ##  ##  ##  ######   #####  ##  ##   ###### ###  ## ######  ## ####    ##   ",
    *b"######   #########  ###  ####    ####    ####    ######  ######     ##    ###    ####      ##   ######    ##    ##  ##  ## ##   ",
    *b"                                                                #####                      ##                       ###  ###    ",
    *b"   ###    ###                           ######    ####    ####    ##                    ##   ## ##   ##                         ",
    *b"                   ###     ###  #####            ## ##   ##  ##                         ##  ##  ##  ##     ##     ##  ####  ##  ",
    *b" ####    ###                            ##  ##   ## ##   ##  ##   ##                    ## ##   ## ##            ##  ##  ##  ## ",
    *b"    ##    ##     ####   ##  ##  #####   ### ##    #####   ####   ##     ######  ######    ##### ####  ##   ##   ##  ##    ##  ##",
    *b" #####    ##    ##  ##  ##  ##  ##  ##  ######                  ##      ##          ##   ##   ## ##  ###   ##    ##  ##  ##  ## ",
    *b"##  ##    ##    ##  ##  ##  ##  ##  ##  ## ###   ######  ###### ##  ##  ##          ##  ##  ### ##  ####  ####    ##  ####  ##  ",
    *b" ######  ####    ####    ###### ##  ##  ##  ##                   ####                   #  ##   #  #####  ####                  ",
    *b"                                                                                           #####      ##   ##                   ",
    *b"  #   #  # # # ### ###     ##      ##      ##     ## ##                   ## ##   ## ##           ## ##   ## ##    ##           ",
    *b"#   #   # # # #  ### ##    ##      ##      ##     ## ##                   ## ##   ## ##           ## ##   ## ##    ##           ",
    *b"  #   #  # # # ### ###     ##      ##   #####     ## ##         #####   #### ##   ## ## ####### #### ##   ## ## #####           ",
    *b"#   #   # # # #  ### ##    ##      ##      ##     ## ##            ##        ##   ## ##      ##      ##   ## ##    ##           ",
    *b"  #   #  # # # ### ###     ##   #####   #####   #### ## ####### #####   #### ##   ## ## #### ## ####### ####### #####   #####   ",
    *b"#   #   # # # #  ### ##    ##      ##      ##     ## ##   ## ##    ##     ## ##   ## ##   ## ##                            ##   ",
    *b"  #   #  # # # ### ###     ##      ##      ##     ## ##   ## ##    ##     ## ##   ## ##   ## ##                            ##   ",
    *b"#   #   # # # #  ### ##    ##      ##      ##     ## ##   ## ##    ##     ## ##   ## ##   ## ##                            ##   ",
    *b"   ##      ##              ##              ##      ##     ## ##   ## ##           ## ##           ## ##           ## ##    ##   ",
    *b"   ##      ##              ##              ##      ##     ## ##   ## ##           ## ##           ## ##           ## ##    ##   ",
    *b"   ##      ##              ##              ##      #####  ## ##   ## ###  ########## ###########  ## ############### ###########",
    *b"   ##      ##              ##              ##      ##     ## ##   ##      ##                      ##                            ",
    *b"   #####################   #####################   #####  ## ###  ######  ## ############### ###  ## ############### ###########",
    *b"                   ##      ##              ##      ##     ## ##           ## ##           ## ##   ## ##           ## ##         ",
    *b"                   ##      ##              ##      ##     ## ##           ## ##           ## ##   ## ##           ## ##         ",
    *b"                   ##      ##              ##      ##     ## ##           ## ##           ## ##   ## ##           ## ##         ",
    *b"  ## ##                   ## ##    ##                     ## ##    ##      ##           ########        ####        ############",
    *b"  ## ##                   ## ##    ##                     ## ##    ##      ##           ########        ####        ############",
    *b"  ## ## ########          ## ##    #####   #####          ## ## ########   ##           ########        ####        ############",
    *b"  ## ##                   ## ##    ##      ##             ## ##            ##           ########        ####        ############",
    *b"########################  ######   #####   #####  ########## ################      #########################        ####        ",
    *b"           ##     ## ##                    ##     ## ##   ## ##    ##              ##   ####################        ####        ",
    *b"           ##     ## ##                    ##     ## ##   ## ##    ##              ##   ####################        ####        ",
    *b"           ##     ## ##                    ##     ## ##   ## ##    ##              ##   ####################        ####        ",
    *b"                                #######                         ######    ###     ###      ###               ##   ####   ####   ",
    *b"         ####   ####### #######  ##  ##          ##  ##  ### ##   ##     ## ##   ## ##    ##                ##   ##     ##  ##  ",
    *b" ### ## ##  ##  ##   ##  ## ##    ##     ######  ##  ## ## ###   ####   ##   ## ##   ##    ##    ######  ###### ##      ##  ##  ",
    *b"## ###  #####   ##       ## ##     ##   ##  ##   ##  ##    ##   ##  ##  ####### ##   ##  #####  ## ## #### ## ########  ##  ##  ",
    *b"##  #   ##  ##  ##       ## ##    ##    ##  ##   ##  ##    ##   ##  ##  ##   ##  ## ##  ##  ##  ## ## #### ## ####      ##  ##  ",
    *b"## ###  #####   ##       ## ##   ##  ## ##  ##   #####     ##    ####    ## ##   ## ##  ##  ##   ######  ######  ##     ##  ##  ",
    *b" ### ## ##      ##       ## ##  #######  ####    ##        ##     ##      ###   ### ###  ####            ##       ####  ##  ##  ",
    *b"        ##                                      ##              ######                                  ##                      ",
    *b"          ##     ##        ##       ###    ##     ##              ###                       #### ####    ####                   ",
    *b"######    ##      ##      ##       ## ##   ##     ##     ###  #  ## ##                      ##   ## ##      ##                  ",
    *b"        ######     ##    ##        ## ##   ##           #  ###   ## ##                      ##   ## ##    ###     ####          ",
    *b"######    ##      ##      ##       ##      ##   ######            ###      ##               ##   ## ##   ##       ####          ",
    *b"          ##     ##        ##      ##      ##            ###  #            ##      ##   ### ##   ## ##   #####    ####          ",
    *b"######                             ##   ## ##     ##    #  ###                           ## ##                    ####          ",
    *b"        ######  ######  ######     ##   ## ##     ##                                      ####                                  ",
    *b"                                   ##    ###                                               ###                                  ",
];
