use wasm_bindgen::prelude::*;

#[cfg(feature="wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn compute(rounds :u32) -> u32 { 
    let mut count : u32 = 0; 
    for _ in 0..rounds { 
        for _ in 0..rounds{
            count += 1;
        }
    }
    return count  
}