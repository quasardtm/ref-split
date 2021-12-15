pub trait RefDestruct: Into<Self::Struct> {
    type Struct;
    fn destruct(self) -> Self::Struct {
        self.into()
    }
}
