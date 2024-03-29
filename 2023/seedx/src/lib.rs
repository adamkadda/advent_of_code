// these modules are defined as submodules of crate 'seedx'
// they are not defined inline

// rust first searches for "src/input_utils.rs"
// rust finds it
#[allow(dead_code)]
mod input_utils;

// rust first searches for "src/part2.rs"
// rust doesn't find it
// rust then searches for  "src/part2/mod.rs"
// rust finds it
#[allow(dead_code)]
mod part2;

#[cfg(test)]
mod tests;