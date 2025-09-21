struct Rec{
    h:i32,
    w:i32,
}


fn main(){
    let r1 = Rec{h:10,w:20};
    println!("{}",calc_area(r1));
    println!("{}",r1.h);
}

fn calc_area(r:Rec) -> i32{
    r.h * r.w
}