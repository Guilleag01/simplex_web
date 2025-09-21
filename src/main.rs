use simplex_web::app::App;

fn main() {
    // let fun = Evaluable::from_string("5 * ((2 - a) * 3)".to_string());
    // println!("{:?}", fun.prefix);

    // let mut val_values: HashMap<String, f32> = HashMap::new();

    // val_values.insert("a".to_string(), 1.0);
    // val_values.insert("b".to_string(), 2.0);
    // val_values.insert("c".to_string(), 3.0);

    // println!("{:?}", fun.evaluate(&val_values));
    yew::Renderer::<App>::new().render();
}
