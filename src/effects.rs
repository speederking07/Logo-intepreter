pub mod effects {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    use crate::turtle::turtle::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Effect {
        Line(f64, f64, f64, f64, Pixel), // x1 y1 x2 y2 color
        Text(String, f64, f64, u32, f64, Pixel), // text x y size rotate color
        Cls,
        Wait(f64), // seconds
        Stop
    }

    fn to_svg(e: Effect) -> String {
        match e {
            Effect::Line(x1, y1, x2, y2, color) => 
                format!("<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='{}' />", x1, y1, x2, y2, color).to_string(),
            Effect::Text(text, x, y, size, rotate, color) => 
                format!("<text style='font: {}px serif; fill: {};transform:translate({}px, {}px) rotate({}rad)'>{}</text>",
                        size, color, x, y, rotate - std::f64::consts::FRAC_PI_2, text).to_string(),
            _ => "".to_string()
        }
    }
    
    pub fn effects_to_canvas(eff: Vec<Effect>) -> Vec<String> {
        eff.into_iter().rev()
            .take_while(|x| *x != Effect::Cls)
            .collect::<Vec<_>>().into_iter()
            .rev()
            .map(|x| to_svg(x))
            .filter(|x| x.as_str() != "")
            .collect()
    }

    pub fn save_as_svg<P: AsRef<Path>>(path: P, eff: Vec<Effect>, size: i32) -> std::io::Result<()>{
        let mut lines = vec![
            format!("<svg viewBox='{} {} {} {}' xmlns='http://www.w3.org/2000/svg'>", -size / 2, -size / 2, size, size).to_string()];
        lines.append(&mut effects_to_canvas(eff));
        lines.push("</svg>".to_string());

        let mut file = File::create(path)?;
        file.write_all(lines.join("\n").as_bytes())?;
        Ok(())
    }
}