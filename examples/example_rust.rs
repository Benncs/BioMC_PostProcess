use bcore::PostProcess;

fn main()
{
    let obj = PostProcess::new("cstr",Some("/home/benjamin/Documents/code/cpp/BioCMA-MCST/results/".to_string()));
    println!("{:?}",obj);
}