use std::io::prelude::*;
use std::fs::File;
use vmfparser::ast::Block;

fn main() {
    let mut f = File::open("./ctf_2fort_d.vmf").expect("could not open file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("could not read file");

    let ast: Vec<Block<String>> = vmfparser::parse(&s).unwrap();

    let mut blocks: Vec<String> = Vec::new();
    println!("Unique Blocks: ");
    print_all_block_names(&mut blocks, &ast);

    let mut properties: Vec<String> = Vec::new();
    println!("Unique Properties: ");
    print_all_property_names(&mut properties, &ast);
}

fn print_all_block_names(mut array: &mut Vec<String>, search: &Vec<Block<String>>) {
    for block in search {
        if !array.contains(&block.name) {
            println!("{}", block.name);
            array.push(block.name.clone());
        }
        if !block.blocks.is_empty() {
            print_all_block_names(&mut array, &block.blocks)
        }
    }
}

fn print_all_property_names(mut array: &mut Vec<String>, search: &Vec<Block<String>>) {
    for block in search {
        for property in &block.props {
            if !array.contains(&property.key) {
                println!("{}", property.key);
                array.push(property.key.clone());
            }
            if !block.blocks.is_empty() {
                print_all_property_names(&mut array, &block.blocks)
            }
        }
    }
}