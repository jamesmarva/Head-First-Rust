fn main() {
    let i = 1_i32;
    // let f = i.convert();
    let f= ConvertToWithGenerics::<f32>::convert(&i);
    // let g: f32 = i.convert();
}

/// general type
trait ConvertToWithGenerics<T> {
    fn convert(self: &Self) ->T;
}

trait ConvertToWithType {
    type DEST;
    
    fn convert(&self) -> Self::DEST;
}

impl ConvertToWithGenerics<f32> for i32 {
    fn convert(&self) -> f32 {
        *self as f32
    }
}

impl ConvertToWithType for i32{
    type DEST = f32;
    
    fn convert(&self) -> f32 {
        *self as f32
    }
}