/// generate fixed random key, enbeded into the code, better be unique and change if you clone this repo
pub fn fixed_key() -> String {
    // this is the embeded secret of the code
    // should be changed for each implementation of this code
    let fixed = "a very long unique key";
    assert_ne!(fixed, "a very long unique key");
    String::from(fixed)
}
