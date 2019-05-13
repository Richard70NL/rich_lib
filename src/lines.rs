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

    /// Set a specified line.
    ///
    /// # Arguments
    ///
    /// * `index` - the index of the line to be set.
    /// * `line` - the line to be set.
    ///
    /// # Example
    ///
    /// ```
    /// use rich_lib::lines::*;
    ///
    /// let s = String::from("line0\nline1\nline2\nline3\nline4")
    ///             .set(1, String::from("lineA"))
    ///             .set(3, String::from("lineB"));
    ///
    /// assert_eq!(s, "line0\nlineA\nline2\nlineB\nline4");
    /// ```
    fn set(self, index: usize, line: String) -> Self;

    /// Inserts a line at a specified location.
    ///
    /// # Arguments
    ///
    /// * `index` - the index of the line to be inserted.
    /// * `line` - the line to be inserted
    ///
    /// # Example
    ///
    /// ```
    /// use rich_lib::lines::*;
    ///
    /// let s = String::from("line0\nline1\nline2")
    ///             .insert(1, String::from("lineA"))
    ///             .insert(3, String::from("lineB"));
    ///
    /// assert_eq!(s, "line0\nlineA\nline1\nlineB\nline2");
    /// ```
    fn insert(self, index: usize, line: String) -> Self;

    /// Removes a line on a specified location.
    ///
    /// # Arguments
    ///
    /// * `index` - the index of the line that needs to be removed.
    ///
    /// # Example
    ///
    /// ```
    /// use rich_lib::lines::*;
    ///
    /// let s = String::from("line0\nline1\nline2").remove(1);
    ///
    /// assert_eq!(s, "line0\nline2");
    /// ```
    fn remove(self, index: usize) -> Self;

    /// Appends a line at the end.
    ///
    /// # Arguments
    ///
    /// * `line` - line to append.
    ///
    /// # Example
    ///
    /// ```
    /// use rich_lib::lines::*;
    ///
    /// let s = String::from("line0\nline1\nline2").append(String::from("line3"));
    ///
    /// assert_eq!(s, "line0\nline1\nline2\nline3");
    /// ```
    fn append(self, line: String) -> Self;

    /// Prepends a line at the beginning.
    ///
    /// # Arguments
    ///
    /// * `line` - line to prepend.
    ///
    /// # Example
    ///
    /// ```
    /// use rich_lib::lines::*;
    ///
    /// let s = String::from("line0\nline1\nline2").prepend(String::from("line3"));
    ///
    /// assert_eq!(s, "line3\nline0\nline1\nline2");
    /// ```
    fn prepend(self, line: String) -> Self;

    /// Returns the first line.
    ///
    /// # Example
    ///
    /// ```
    /// use rich_lib::lines::*;
    ///
    /// let s = String::from("line0\nline1\nline2").first();
    ///
    /// assert_eq!(s, "line0");
    /// ```
    fn first(&self) -> String;

    /// Returns the last line.
    ///
    /// # Example
    ///
    /// ```
    /// use rich_lib::lines::*;
    ///
    /// let s = String::from("line0\nline1\nline2").last();
    ///
    /// assert_eq!(s, "line2");
    /// ```
    fn last(&self) -> String;

    /// Removes the first line.
    ///
    /// # Example
    ///
    /// ```
    /// use rich_lib::lines::*;
    ///
    /// let s = String::from("line0\nline1\nline2").remove_first();
    ///
    /// assert_eq!(s, "line1\nline2");
    /// ```
    fn remove_first(self) -> Self;

    /// Removes the last line.
    ///
    /// # Example
    ///
    /// ```
    /// use rich_lib::lines::*;
    ///
    /// let s = String::from("line0\nline1\nline2").remove_last();
    ///
    /// assert_eq!(s, "line0\nline1");
    /// ```
    fn remove_last(self) -> Self;
}

/************************************************************************************************/

/// The `SplitLines` trait contains all functions to be implemented that splits an object (String)
/// into separate lines.
pub trait SplitLines {
    /// Splits to multiple lines.
    ///
    /// # Example
    ///
    /// ```
    /// use rich_lib::lines::*;
    ///
    /// let s = String::from("line0\nline1\nline2").split();
    /// let v = vec![
    ///     String::from("line0"),
    ///     String::from("line1"),
    ///     String::from("line2")
    /// ];
    ///
    /// assert_eq!(s, v);
    /// ```
    fn split(&self) -> StringLines;
}

/************************************************************************************************/

/// The `MergeLines` trait contains all functions to be implemented that will join separate lines
/// into a single object (String).
pub trait MergeLines {
    /// Merges multiple lines.
    ///
    /// # Example
    ///
    /// ```
    /// use rich_lib::lines::*;
    ///
    /// let v = vec![
    ///     String::from("line0"),
    ///     String::from("line1"),
    ///     String::from("line2")
    /// ];
    /// let s = v.merge();
    ///
    /// assert_eq!(s, "line0\nline1\nline2");
    /// ```
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
