use image::GenericImageView;


fn main() {
    let img = image::open("misc/img.jpg").expect("Échec du chargement");
    println!("Dimensions: {:?}", img.dimensions());
    println!("Type de couleurs {:?}",img.color());
}   
