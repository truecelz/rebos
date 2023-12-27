use std::io;
use piglog::prelude::*;

pub fn tidy_up() -> Result<(), io::Error> {
    let deleted = clean_dups(false)?;
    let aligned = align(false)?;

    piglog::success!("Deleted {deleted} duplicate generations!");
    piglog::success!("Aligned {aligned} generations!");

    return Ok(());
}

pub fn clean_dups(verbose: bool) -> Result<usize, io::Error> {
    let mut deleted: usize = 0; // The amount of deleted generations.

    let mut gen_nums = super::list_gen_nums()?;
    gen_nums.sort();

    let mut comparison: Option<super::Generation> = None;

    for i in gen_nums {
        let generation = super::get_gen_from_usize(i)?;

        if let Some(ref s) = comparison {
            if generation == *s {
                if verbose {
                    piglog::info!("Deleting generation: {i}");
                }

                match super::delete(i) {
                    Ok(_) => (),
                    Err(e) => {
                        piglog::fatal!("Failed to delete generation: {i}");

                        return Err(e);
                    },
                };

                deleted += 1;
            }

            else {
                comparison = Some(generation);
            }
        }

        else {
            comparison = Some(generation);
        }
    }

    return Ok(deleted);
}

pub fn align(verbose: bool) -> Result<usize, io::Error> {
    let mut moved: usize = 0; // The amount of moved generations.

    let mut gen_nums = super::list_gen_nums()?;
    gen_nums.sort();

    let mut comparison: usize = 0;

    for i in gen_nums {
        let new_number = comparison + 1;

        if i > new_number {
            if verbose {
                piglog::info!("Moving: {i} -> {new_number}");
            }

            match super::move_gen(i, new_number) {
                Ok(_) => (),
                Err(e) => {
                    piglog::fatal!("Failed to move generation {i} to {new_number}!");

                    return Err(e);
                },
            };

            moved += 1;
        }

        comparison += 1;
    }

    return Ok(moved);
}
