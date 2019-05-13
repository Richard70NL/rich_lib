//! `Lines` is a module that contains various traits to easily work with lines in a String.

/************************************************************************************************/
/************************************************************************************************/
/************************************************************************************************/

/// A simple vector of strings.
pub type StringLines = Vec<String>;

/************************************************************************************************/
/************************************************************************************************/
/************************************************************************************************/

/// The `Lines` trait contains all functions to be implemented.
pub trait Lines {
    /// Returns the total amount of lines.
    ///
    /// # Example
    ///
    /// ```
    /// use rich_lib::lines::*;
    ///
    /// let s = String::from("line0\nline1\nline2");
    /// assert_eq!(s.count(), 3);
    ///
    /// let s = String::new();
    /// assert_eq!(s.count(), 0);
    /// ```
    fn count(&self) -> usize;

    /// Returns a specified line.
    ///
    /// # Arguments
    ///
    /// * `index` - the index of the line to return.
    ///
    /// # Example
    ///
    /// ```
    /// use rich_lib::lines::*;
    ///
    /// let s = String::from("line0\nline1\nline2");
    /// assert_eq!(s.get(1), "line1");
    /// ```
    fn get(&self, index: usize) -> String;

    fn set(self, index: usize, line: String) -> Self;

    fn insert(self, index: usize, line: String) -> Self;

    fn remove(self, index: usize) -> Self;

    fn append(self, line: String) -> Self;

    fn prepend(self, line: String) -> Self;

    fn first(&self) -> String;

    fn last(&self) -> String;

    fn remove_first(self) -> Self;

    fn remove_last(self) -> Self;
}

/************************************************************************************************/

/// The `SplitLines` trait contains all functions to be implemented that splits an object (String)
/// into separate lines.
pub trait SplitLines {
    fn split(&self) -> StringLines;
}

/************************************************************************************************/

/// The `MergeLines` trait contains all functions to be implemented that will join separate lines
/// into a single object (String).
pub trait MergeLines {
    fn merge(&self) -> String;
}

/************************************************************************************************/
/************************************************************************************************/
/************************************************************************************************/

impl Lines for String {
    /*------------------------------------------------------------------------------------------*/

    fn count(&self) -> usize {
        self.split().len()
    }

    /*------------------------------------------------------------------------------------------*/

    fn get(&self, index: usize) -> String {
        let sl = self.split();
        sl[index].clone()
    }

    /*------------------------------------------------------------------------------------------*/

    fn set(self, index: usize, line: String) -> Self {
        let mut sl = self.split();
        sl[index] = line;
        sl.merge()
    }

    /*------------------------------------------------------------------------------------------*/

    fn insert(self, index: usize, line: String) -> Self {
        let mut sl = self.split();
        sl.insert(index, line);
        sl.merge()
    }

    /*------------------------------------------------------------------------------------------*/

    fn remove(self, index: usize) -> Self {
        let mut sl = self.split();
        sl.remove(index);
        sl.merge()
    }

    /*------------------------------------------------------------------------------------------*/

    fn append(self, line: String) -> Self {
        let mut sl = self.split();
        sl.insert(sl.len(), line);
        sl.merge()
    }

    /*------------------------------------------------------------------------------------------*/

    fn prepend(self, line: String) -> Self {
        let mut sl = self.split();
        sl.insert(0, line);
        sl.merge()
    }

    /*------------------------------------------------------------------------------------------*/

    fn first(&self) -> String {
        let sl = self.split();
        sl[0].clone()
    }

    /*------------------------------------------------------------------------------------------*/

    fn last(&self) -> String {
        let sl = self.split();
        sl[sl.len() - 1].clone()
    }

    /*------------------------------------------------------------------------------------------*/

    fn remove_first(self) -> Self {
        let mut sl = self.split();
        sl.remove(0);
        sl.merge()
    }

    /*------------------------------------------------------------------------------------------*/

    fn remove_last(self) -> Self {
        let mut sl = self.split();
        sl.remove(sl.len() - 1);
        sl.merge()
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/

impl SplitLines for String {
    fn split(&self) -> StringLines {
        self.lines().map(String::from).collect()
    }
}

/************************************************************************************************/

impl MergeLines for StringLines {
    fn merge(&self) -> String {
        let mut buffer = String::new();

        for line in self.iter() {
            if !buffer.is_empty() {
                buffer.push('\n');
            }
            buffer.push_str(line);
        }

        buffer
    }
}

/************************************************************************************************/
/************************************************************************************************/
/************************************************************************************************/
