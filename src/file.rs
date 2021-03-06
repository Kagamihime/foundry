//! This module contains the functions and methods that enable to load and
//! save files containing grid data.
//! For now it supports two internal file formats : "Resizable Life"
//! and "Toroidal Life".

use std::collections::LinkedList;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;

use error::FileParsingErrorKind;
use Grid;

impl Grid {
    /// Returns a new `Grid` encoded within a file located at `path`.
    ///
    /// # Errors
    ///
    /// If there is an IO error or the file isn't a valid life file,
    /// an error of the type `FileParsingErrorKind` will be returned.
    pub fn from_file(path: &str) -> Result<Grid, FileParsingErrorKind> {
        // Open and read file
        let mut f = File::open(path)?;
        let mut lines = String::new();
        f.read_to_string(&mut lines)?;

        // Remove leading and trailing whitespaces and then remove blank lines
        let lines = lines
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty());
        // Turn the iterator into a LinkedList<&str>
        let lines: LinkedList<&str> = LinkedList::from_iter(lines);

        // Check if file is valid
        valid_life_file(&lines)?;

        // The first line should indicate the format to be used
        if *lines.front().ok_or(FileParsingErrorKind::IncompleteFile)? == "#Resizable Life" {
            load_resizable_life(lines)
        } else if *lines.front().ok_or(FileParsingErrorKind::IncompleteFile)? == "#Toroidal Life" {
            load_toroidal_life(lines)
        } else {
            Err(FileParsingErrorKind::UnknownFormat)
        }
    }

    /// Writes the `Grid` into a file located at `path`.
    ///
    /// # Errors
    ///
    /// If there is an IO error, an error of the type `io::Error`
    /// will be returned.
    pub fn save_life_grid(&self, path: &str) -> Result<(), io::Error> {
        let mut lines: LinkedList<String> = LinkedList::new();

        // Recenter the `Grid`
        let mut grid = self.clone();
        if !self.is_toroidal() {
            grid.recenter_pattern(0);
        }

        // Put format
        if grid.is_toroidal() {
            lines.push_back("#Toroidal Life".to_string());
        } else {
            lines.push_back("#Resizable Life".to_string());
        }

        // Put ruleset
        let mut survival_ruleset = String::new();
        let mut birth_ruleset = String::new();
        for n in grid.survival.read().unwrap().iter() {
            survival_ruleset.push_str(&n.to_string());
        }
        for n in grid.birth.read().unwrap().iter() {
            birth_ruleset.push_str(&n.to_string());
        }
        lines.push_back(format!("#R {}/{}", survival_ruleset, birth_ruleset));

        // Put grid size if toroidal
        let width = grid.get_width();
        let height = grid.get_height();
        if grid.is_toroidal() {
            lines.push_back(format!("#S {} {}", width, height));
        }

        // Put living cells coords
        for y in 0..height {
            for x in 0..width {
                if grid.get_cell_state(x as i64, y as i64) == 255 {
                    lines.push_back(format!("{} {}", x, y));
                }
            }
        }

        // Write lines to a file
        let mut f = File::create(path)?;
        for line in lines {
            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}

fn valid_life_file(lines_ref: &LinkedList<&str>) -> Result<(), FileParsingErrorKind> {
    let mut lines = lines_ref.clone(); // Make a copy of lines_ref so it can be modified
                                       // If "lines" is empty then the file is empty
    let format_line = lines
        .pop_front()
        .ok_or(FileParsingErrorKind::IncompleteFile)?;

    if format_line == "#Resizable Life" {
        match valid_resizable_life(lines) {
            Err(err) => Err(err),
            Ok(()) => Ok(()),
        }
    } else if format_line == "#Toroidal Life" {
        match valid_toroidal_life(lines) {
            Err(err) => Err(err),
            Ok(()) => Ok(()),
        }
    } else {
        Err(FileParsingErrorKind::UnknownFormat)
    }
}

fn valid_resizable_life(lines: LinkedList<&str>) -> Result<(), FileParsingErrorKind> {
    let mut lines = lines; // Make lines mutable

    // If any, check description
    while lines
        .front()
        .ok_or(FileParsingErrorKind::IncompleteFile)?
        .starts_with("#D")
    {
        lines
            .pop_front()
            .ok_or(FileParsingErrorKind::IncompleteFile)?;
    }

    //Check ruleset
    if *lines.front().ok_or(FileParsingErrorKind::IncompleteFile)? != "#N" {
        let ruleset_line = lines
            .pop_front()
            .ok_or(FileParsingErrorKind::IncompleteFile)?;
        if !ruleset_line.starts_with("#R") || ruleset_line.split_whitespace().count() != 2 {
            return Err(FileParsingErrorKind::RuleParsingError);
        }
        let ruleset = ruleset_line
            .split_whitespace()
            .filter(|s| *s != "#R")
            .next()
            .ok_or(FileParsingErrorKind::IncompleteFile)?; // Without .next() there is a type error with split method
        if ruleset.split('/').count() != 2 {
            return Err(FileParsingErrorKind::RuleParsingError);
        }
        let ruleset: Vec<&str> = ruleset.split('/').collect();
        // ruleset[0] is the survival ruleset
        for c in ruleset[0].chars() {
            match c.to_digit(10) {
                None => return Err(FileParsingErrorKind::RuleParsingError),
                Some(n) if n == 9 => return Err(FileParsingErrorKind::RuleParsingError),
                Some(_) => {}
            }
        }
        // ruleset[1] is the birth ruleset
        for c in ruleset[1].chars() {
            match c.to_digit(10) {
                None => return Err(FileParsingErrorKind::RuleParsingError),
                Some(n) if n == 9 => return Err(FileParsingErrorKind::RuleParsingError),
                Some(_) => {}
            }
        }
    } else {
        lines
            .pop_front()
            .ok_or(FileParsingErrorKind::IncompleteFile)?;
    }

    // If no "coords" lines return an error
    if lines.is_empty() {
        return Err(FileParsingErrorKind::IncompleteFile);
    }

    // Check "coords" lines
    while !lines.is_empty() {
        if lines
            .front()
            .ok_or(FileParsingErrorKind::IncompleteFile)?
            .split_whitespace()
            .count()
            != 2
        {
            return Err(FileParsingErrorKind::CoordParsingError);
        }
        let coords: Vec<&str> = lines
            .pop_front()
            .ok_or(FileParsingErrorKind::IncompleteFile)?
            .split_whitespace()
            .collect();
        coords[0].parse::<usize>()?;
        coords[1].parse::<usize>()?;
    }

    // If all tests are passed...
    Ok(())
}

fn valid_toroidal_life(lines: LinkedList<&str>) -> Result<(), FileParsingErrorKind> {
    let mut lines = lines; // Make lines mutable

    // If any, check description
    while lines
        .front()
        .ok_or(FileParsingErrorKind::IncompleteFile)?
        .starts_with("#D")
    {
        lines
            .pop_front()
            .ok_or(FileParsingErrorKind::IncompleteFile)?;
    }

    // Check ruleset
    if *lines.front().ok_or(FileParsingErrorKind::IncompleteFile)? != "#N" {
        let ruleset_line = lines
            .pop_front()
            .ok_or(FileParsingErrorKind::IncompleteFile)?;
        if !ruleset_line.starts_with("#R") || ruleset_line.split_whitespace().count() != 2 {
            return Err(FileParsingErrorKind::RuleParsingError);
        }
        let ruleset = ruleset_line
            .split_whitespace()
            .filter(|s| *s != "#R")
            .next()
            .ok_or(FileParsingErrorKind::IncompleteFile)?; // Without .next() there is a type error with split method
        if ruleset.split('/').count() != 2 {
            return Err(FileParsingErrorKind::RuleParsingError);
        }
        let ruleset: Vec<&str> = ruleset.split('/').collect();
        // ruleset[0] is the survival ruleset
        for c in ruleset[0].chars() {
            match c.to_digit(10) {
                None => return Err(FileParsingErrorKind::RuleParsingError),
                Some(n) if n == 9 => return Err(FileParsingErrorKind::RuleParsingError),
                Some(_) => {}
            }
        }
        // ruleset[1] is the birth ruleset
        for c in ruleset[1].chars() {
            match c.to_digit(10) {
                None => return Err(FileParsingErrorKind::RuleParsingError),
                Some(n) if n == 9 => return Err(FileParsingErrorKind::RuleParsingError),
                Some(_) => {}
            }
        }
    } else {
        lines
            .pop_front()
            .ok_or(FileParsingErrorKind::IncompleteFile)?;
    }

    // Check grid size specification (#S <width> <height>)
    let grid_size_line = lines
        .pop_front()
        .ok_or(FileParsingErrorKind::IncompleteFile)?;
    if !grid_size_line.starts_with("#S") || grid_size_line.split_whitespace().count() != 3 {
        return Err(FileParsingErrorKind::CoordParsingError);
    }
    let grid_size: Vec<&str> = grid_size_line
        .split_whitespace()
        .filter(|s| *s != "#S")
        .collect();
    grid_size[0].parse::<usize>()?;
    grid_size[1].parse::<usize>()?;

    // If no "coords" lines return an error
    if lines.is_empty() {
        return Err(FileParsingErrorKind::IncompleteFile);
    }

    // Check "coords" lines
    while !lines.is_empty() {
        if lines
            .front()
            .ok_or(FileParsingErrorKind::IncompleteFile)?
            .split_whitespace()
            .count()
            != 2
        {
            return Err(FileParsingErrorKind::CoordParsingError);
        }
        let coords: Vec<&str> = lines
            .pop_front()
            .ok_or(FileParsingErrorKind::IncompleteFile)?
            .split_whitespace()
            .collect();
        coords[0].parse::<usize>()?;
        coords[1].parse::<usize>()?;
    }

    // If all tests are passed...
    Ok(())
}

fn load_resizable_life(lines: LinkedList<&str>) -> Result<Grid, FileParsingErrorKind> {
    let mut lines = lines; // Make lines mutable

    // Get file format
    let frmt = lines.pop_front().unwrap();

    // Skip description
    while lines.front().unwrap().starts_with("#D") {
        lines.pop_front().unwrap();
    }

    // Get ruleset
    let mut srvl: Vec<u32> = Vec::new();
    let mut brth: Vec<u32> = Vec::new();
    if *lines.front().unwrap() == "#N" {
        lines.pop_front().unwrap();
        srvl = vec![2, 3];
        brth = vec![3];
    } else {
        let ruleset_line = lines.pop_front().unwrap();
        let ruleset = ruleset_line
            .split_whitespace()
            .filter(|s| *s != "#R")
            .next()
            .unwrap(); // Without .next().unwrap() there is a type error with split method
        let ruleset: Vec<&str> = ruleset.split("/").collect();
        let survival_ruleset = ruleset[0].chars();
        let birth_ruleset = ruleset[1].chars();
        for c in survival_ruleset {
            srvl.push(c.to_digit(10).unwrap() as u32);
        }
        for c in birth_ruleset {
            brth.push(c.to_digit(10).unwrap() as u32);
        }
    }
    // Sort and remove duplicated rules
    srvl.sort();
    srvl.dedup();
    brth.sort();
    brth.dedup();

    // Guess the "cells" size
    let mut file_coords: Vec<(usize, usize)> = Vec::new();
    while !lines.is_empty() {
        let coords_str: Vec<&str> = lines.pop_front().unwrap().split_whitespace().collect();
        let coords: (usize, usize) = (
            coords_str[0].parse().unwrap(),
            coords_str[1].parse().unwrap(),
        );
        file_coords.push(coords);
    }
    let (width, height) = guess_pattern_size(&file_coords);

    // Make CA grid
    let mut grid = Grid::new(&frmt.to_string(), false, &srvl, &brth, width, height);

    // Set to true the cells that are alive
    for (x, y) in file_coords {
        grid.set_cell_state(x, y, 255)?;
    }

    // Return CA grid
    Ok(grid)
}

// Works like Life 1.06 except there is a #S <width> <height> before the cells "coords"
fn load_toroidal_life(lines: LinkedList<&str>) -> Result<Grid, FileParsingErrorKind> {
    let mut lines = lines; // Make lines mutable

    // Get file format
    let frmt = lines.pop_front().unwrap();

    // Skip description
    while lines.front().unwrap().starts_with("#D") {
        lines.pop_front().unwrap();
    }

    // Get ruleset
    let mut srvl: Vec<u32> = Vec::new();
    let mut brth: Vec<u32> = Vec::new();
    if *lines.front().unwrap() == "#N" {
        lines.pop_front().unwrap();
        srvl = vec![2, 3];
        brth = vec![3];
    } else {
        let ruleset_line = lines.pop_front().unwrap();
        let ruleset = ruleset_line
            .split_whitespace()
            .filter(|s| *s != "#R")
            .next()
            .unwrap(); // Without .next().unwrap() there is a type error with split method
        let ruleset: Vec<&str> = ruleset.split("/").collect();
        let survival_ruleset = ruleset[0].chars();
        let birth_ruleset = ruleset[1].chars();
        for c in survival_ruleset {
            srvl.push(c.to_digit(10).unwrap() as u32);
        }
        for c in birth_ruleset {
            brth.push(c.to_digit(10).unwrap() as u32);
        }
    }
    // Sort and remove duplicated rules
    srvl.sort();
    srvl.dedup();
    brth.sort();
    brth.dedup();

    // Get the grid size
    let grid_size_line_terms: Vec<&str> = lines
        .pop_front()
        .unwrap()
        .split_whitespace()
        .filter(|s| *s != "#S")
        .collect();
    let (width, height): (usize, usize) = (
        grid_size_line_terms[0].parse().unwrap(),
        grid_size_line_terms[1].parse().unwrap(),
    );

    // Make CA grid
    let mut grid = Grid::new(&frmt.to_string(), true, &srvl, &brth, width, height);

    // Get the coordinates from the file
    let mut file_coords: Vec<(usize, usize)> = Vec::new();
    while !lines.is_empty() {
        let coords_str: Vec<&str> = lines.pop_front().unwrap().split_whitespace().collect();
        let coords: (usize, usize) = (
            coords_str[0].parse().unwrap(),
            coords_str[1].parse().unwrap(),
        );
        file_coords.push(coords);
    }

    // Set to true the cells that are alive
    for (x, y) in file_coords {
        grid.set_cell_state(x, y, 255)?;
    }

    // Return CA grid
    Ok(grid)
}

fn guess_pattern_size(coords: &[(usize, usize)]) -> (usize, usize) {
    let (mut lim_x, mut lim_y): (usize, usize) = (0, 0);

    for &(x, y) in coords {
        if x > lim_x {
            lim_x = x;
        }
        if y > lim_y {
            lim_y = y;
        }
    }
    // The "+ 1"s are here because because the "coords" start at 0
    (lim_x + 1, lim_y + 1)
}
