pub trait Observer {
    fn update(&self);
}

pub trait Subject<'a, O: Observer> {
    fn attach(&mut self, observer: &'a O);
    fn detach(&mut self, observer: &'a O);
    fn notify(&self);
}
