mod debug;
use debug::PerformanceTimer;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn main() {
    {
        let timer = PerformanceTimer::new();
        //println!("");
    }

}
