use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::Data;

#[proc_macro_derive(Vertex)]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = ast.ident;
    let expanded = match ast.data {
        Data::Struct(statment) => {
            let mut attributes = Vec::new();
            for (index, field) in statment.fields.into_iter().enumerate() {
                let format = match field
                    .ty
                    .to_token_stream()
                    .to_string()
                    .replace(" ", "")
                    .as_str()
                {
                    "[f32;2]" => "Float32x2",
                    "[f32;3]" => "Float32x3",
                    "[f32;4]" => "Float32x4",
                    "u32" => "Uint32",
                    "i32" => "Sint32",
                    ty => panic!("Type '{ty}' not recognized for a vertex"),
                };
                let ident = syn::Ident::new(format, proc_macro2::Span::call_site());
                let index_lit =
                    syn::LitInt::new(&index.to_string(), proc_macro2::Span::call_site());
                attributes.push(quote! {
                    #index_lit => #ident
                });
            }
            quote! {
                unsafe impl bytemuck::Pod for #ident {}
                unsafe impl bytemuck::Zeroable for #ident {}
                impl GpuVertex for #ident {
                    const VERTEX_LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Self>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &vertex_attr_array![
                            #(#attributes),*
                        ],
                    };
                    const INSTANCE_LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Self>() as u64,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &vertex_attr_array![
                            #(#attributes),*
                        ],
                    };
                }
            }
        }
        _ => panic!("Macro only supported on structs"),
    };

    eprintln!("Expanded macro:\n{}", expanded);
    expanded.into()
}
