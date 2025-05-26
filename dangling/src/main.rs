fn main() {
   // let ref_to_nothin = dangle();
   let _not_ref_but_smtin = not_dangle();
}

fn not_dangle() -> String {
    let s = String::from("hello");

    s
}

//fn dangle() -> &String {
//    let s = String::from("hello");
//
//    &s
//}
