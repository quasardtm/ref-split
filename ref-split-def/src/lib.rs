pub trait RefSplit: Into<Self::Struct> {
    type Struct;
    fn split(self) -> Self::Struct {
        self.into()
    }
}
