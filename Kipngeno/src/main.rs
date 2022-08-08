struct Rectangle {
    height: i32,
    width: i32,
}
impl Rectangle {
    fn perimeter($self) -> i32 {
        (self.height * 2) + (self.width + 2)
    }
}
