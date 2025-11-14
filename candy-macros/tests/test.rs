#[cfg(test)]
mod tests {
    use candy_macros::Vertex;

    #[test]
    fn text_derive_vertex() {
        #[derive(Vertex)]
        struct Test {
            pos: [f32; 4],
        }
    }
}
