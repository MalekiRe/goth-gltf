fn main() {
    let bytes = std::fs::read("Malek.vrm").unwrap();
    let (gltf, b): (
        goth_gltf::Gltf<goth_gltf::default_extensions::Extensions>,
        _,
    ) = goth_gltf::Gltf::from_bytes(&bytes).unwrap();
    println!("{:#?}", gltf);
}
