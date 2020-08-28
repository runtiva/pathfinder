use pathfinder_canvas::{Canvas, CanvasFontContext, CanvasRenderingContext2D, Path2D};
use pathfinder_color::{ColorF}; //, rgbau, rgbaf};
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_geometry::vector::Vector2I;
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererMode, RendererOptions};
use pathfinder_renderer::options::BuildOptions;
use pathfinder_resources::embedded::EmbeddedResourceLoader;

struct PdfPathfinderDevice {
    canvas: CanvasRenderingContext2D,
}

impl PdfPathfinderDevice {
    pub fn new(gl_device: GLDevice, window_size: Vector2I) -> PdfPathfinderDevice {
        let mode = RendererMode::default_for_device(&gl_device);

        let options = RendererOptions {
            dest: DestFramebuffer::full_window(window_size), // framebuffer_size),
            background_color: Some(ColorF::white()),
            show_debug_ui: false, // ..RendererOptions::default()
        };
    
        let resource_loader = EmbeddedResourceLoader::new();
        let mut renderer = Renderer::new(gl_device, &resource_loader, mode, options);
    
        let font_context = CanvasFontContext::from_system_source();
        let mut canvas = Canvas::new(/*framebuffer_size*/window_size.to_f32()).get_context_2d(font_context);
    
        PdfPathfinderDevice {
            canvas,
        }
    }

    /// General Graphics State              w, J, j, M, d, ri, i, gs
    /// Line Width
    /// The `w` operator sets the line width in the graphics state (see 8.4.3.2, "Line Width").
    pub fn set_line_width(&mut self, line_width: f32) -> Result<(), PdfRenderError> {
        if line_width < 0.0 {
            return Err(PdfRenderError::ProcessorError("w".to_string(), format!("{}", line_width)));
        }
        
        {
            let mut gs = self.gs.borrow_mut();
            (*gs).line_width = line_width;
        }
        
        Ok(())
    }

    /// Line Cap
    /// The `J` operator sets the line cap style in the graphics state (see 8.4.3.3, "Line Cap Style").
    #[allow(non_snake_case)]
    pub fn set_line_cap_style(&self, line_cap_style: i32) -> Result<(), PdfRenderError> {
        let style: LineCapStyle = line_cap_style.try_into()?;
        
        {
            let mut gs = self.gs.borrow_mut();
            (*gs).line_cap_style = style;
        }
        
        Ok(())
    }

    /// Line Cap
    /// The `j` operator sets the line join style in the graphics state (see 8.4.3.4, "Line Join Style").
    pub fn gs_operator_j(&self, line_join_style: i32) -> Result<(), PdfRenderError> {
        let style: LineJoinStyle = line_join_style.try_into()?;
        
        {
            let mut gs = self.gs.borrow_mut();
            (*gs).line_join_style = style;
        }
        
        Ok(())
    }

    /// Miter Limit
    /// The `M` operator sets the miter limit in the graphics state (see 8.4.3.5, "Miter Limit").
    #[allow(non_snake_case)]
    pub fn gs_operator_M(&self, miter_limit: f32) -> Result<(), PdfRenderError> {
        if miter_limit < 0.0 {
            return Err(PdfRenderError::ProcessorError("M".to_string(), format!("{}", miter_limit)));
        }
        
        {
            let mut gs = self.gs.borrow_mut();
            (*gs).miter_limit = miter_limit;
        }
        
        Ok(())
    }

    /// dashArray dashPhase
    /// The `d` operator sets the line dash pattern in the graphics state (see 8.4.3.6, "Line Dash Pattern").
    pub fn gs_operator_d(&self, dash_array: Vec<f32>, phase: f32) -> Result<(), PdfRenderError> {

        let pattern = LineDashPattern::new(dash_array, phase);
        
        {
            let mut gs = self.gs.borrow_mut();
            (*gs).line_dash_pattern = pattern;
        }
        
        Ok(())
    }

    /// Colour Rendering Intent
    /// (PDF 1.1) The `ri` operator sets the colour rendering intent in the graphics state (see 8.6.5.8, "Rendering Intents").
    pub fn gs_operator_ri(&self, rendering_intent: i32) -> Result<(), PdfRenderError> {
        let rendering_intent: RenderingIntent = rendering_intent.try_into()?;
        
        {
            let mut gs = self.gs.borrow_mut();
            (*gs).rendering_intent = rendering_intent;
        }
        
        Ok(())
    }

    /// Flatness tolerance
    /// The `i` operator sets the flatness tolerance in the graphics state (see 10.6.2, "Flatness Tolerance").
    /// `flatness` is a number in the range of 0 to 100; a value of 0 shall specify the output device's default flatness tolerance.  
    pub fn gs_operator_i(&self, flatness: f32) -> Result<(), PdfRenderError> {

        if flatness < 0.0 || flatness > 100.0 {
            return Err(PdfRenderError::ProcessorError("i".to_string(), format!("{}", flatness)));
        }
        
        {
            let mut gs = self.gs.borrow_mut();
            (*gs).flatness = flatness;
        }
        
        Ok(())
    }

    /// Graphics State Dictionary
    /// (PDF 1.2) Set the specified parameters in the graphics state. dictName shall be the name of a graphics state parameter
    //// dictionary in the ExtGState subdictionary of the current resource dictionary (see the next sub-clause).  
    pub fn gs_operator_gs(&self, gs_name: &str) -> Result<(), PdfRenderError> {

        if gs_name.is_empty() {
            return Err(PdfRenderError::ProcessorError("gs".to_string(), "GS dictionary name missing".to_string()));
        }
        
        panic!("Not Implemented");

        // TODO: Process GS Dictionary
        
        //Ok(())
    }






    // Graphics State Operators
    pub fn set_fill_color(&mut self, color: ColorF) {

    }

    pub fn set_stroke_color(&mut self, color: ColorF) {
    
    }

    // Path Construction
    pub fn rect(&mut self, xy: Vector2F, wh: Vector2F) {

    }

    pub fn move_to(&mut self, to: Vector2F) {

    }

    pub fn line_to(&mut self, to: Vector2F) {

    }

    pub fn bezier_curve_to(&mut self, ctrl1: Vector2F, ctrl2: Vector2F, to: Vector2F) {

    }

    pub fn close_path(&mut self) {

    }

    // Path Painting
    pub fn fill_path(&mut self) {

    }

    pub fn stroke_path(&mut self) {

    }



}