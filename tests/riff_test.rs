use create_vox::riff::VoxString;

#[test]
fn riff_string(){
    let content = &[2, 0, 0, 0, 104, 105];
    let my_string = VoxString::read(content).content;

    assert_eq!(String::from("hi"), my_string);
}

#[test]
#[should_panic]
fn riff_string_fail(){
    let content = &[2, 0, 0, 0, 104, 105];
    let my_string = VoxString::read(content).content;

    assert_eq!(String::from("HI"), my_string);
}