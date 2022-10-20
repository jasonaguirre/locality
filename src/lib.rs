pub struct TwoDArray <T>{
    pub matrix: Vec<T>,
    pub height: u32,
    pub width: u32,
    /*
    We plan on implementing our data structure with a 1D array and we will need to use a formula to flatten a 2D array
    into a 1D array and convert a 1D Array into a 2D Array
     */

}

impl <T> TwoDArray<T> {
     /// Constructor for our struct that will initialize an empty TwoDArray of type T
     /// Will return a TwoDArray of type T, the 
    pub fn new(height:u32, width:u32, input: Vec<T>) -> TwoDArray<T> {
        //! Since there are no constructors in Rust, this is our constructor, which is a new function (programming idiom)
        if input.len() != (height * width) as usize {
            eprintln!("Error: You are trying to input a vector not consistent with the number of rows and columns");
        }
        let new2D: TwoDArray<T> = TwoDArray { 
            matrix: input, 
            height: height, 
            width: width 
        };
        
        return new2D;
    }

    /// Constructor if the data is in row major
    // pub fn input_with_row_major(height:u32, width:u32, input: Vec<T>) -> TwoDArray<T> {
    //     // a lot of error checking
    //     let new2D: TwoDArray<T> = TwoDArray { 
    //         matrix: input, 
    //         height: height, 
    //         width: width 
    //     };
    //     return new2D;
    // }

    /// Constructor if the data is in column major
    // pub fn input_with_column_major(height:u32, width:u32, input: Vec<T>) -> TwoDArray<T> {
    //     let new2D: TwoDArray<T> = TwoDArray { matrix: input, 
    //         height: height, 
    //         width: width 
    //     };
    //     let mut starter: u32 = 0;
    //     let mut ptr: u32 = 0;
    //     let skipper:u32 = height;
    //     let mut blank:Vec<_> = Vec::new();
    //     while new2D.matrix.len() != blank.len() {
    //         blank.push(&new2D.matrix[ptr as usize]);
    //         ptr += skipper;
    //         if ptr >= new2D.matrix.len() as u32 {
    //             starter += 1;
    //             ptr = starter;
    //         }
    //     }

    //     return new2D;
    // }

    /// This traverses the TwoDArray from left to right and top to bottom (walks left on one row, then down a row, walks that row, etc.)
    /// This would return a vector that is listing out all of the elements in row-major in a single vector of type T
    // TEST if the edges of the rows
    pub fn rowMajor(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.matrix.iter().enumerate().map(|x| {
            let c = x.0 % self.width as usize;
            let r = (x.0) / self.width as usize;
            // let newValue: &T = &self.searchForElement(r as u32, c as u32);
            // let newValue = Box::new(self.searchForElement(r as u32, c as u32));
            return (r, c, x.1);
        })
        // We would just have to traverse this array as 1D since 1D arrays are by default row major
        // call the iterator for each col
            // this creates an iterator for each column and skip by allows you to skip by 0 until numCols
    }
    /// This traverses the TwoDArray from top to bottom and left to right (walks down a column, moves to the column to the right, walk down that column, etc.)
    /// This would return a vector that is listing all of the elements in column-major in a single vector of type T
    /// 
    pub fn columnMajor(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        // let tuple = (0..self.width).flat_map(|x| {self.matrix.iter().skip(x as usize).step_by(self.width as usize)}).enumerate();
        let tuple = (0..self.width).flat_map(|x| {
            self.matrix.iter()
            .enumerate()
            .skip(x as usize)
            .step_by(self.width as usize)
            .map(|val| (self.getRCbyIndex(val.0 as u32), val.1 as &T)).into_iter()});
        // let row = tuple...
        let new_iter = tuple.map(
            /*  original tuple  vvStringvv  vvvv u32vvvv     */
                    |original| (original.0.0 as usize, original.0.1 as usize, original.1)
                );
        return new_iter; 
    }
    // / We can figure out how to traverse by column major with another formula using the numbers of rows and columns  

    /// This searches for an element in the TwoDArray with a row index (r) and a column index (c)
    /// Returns the element at r, c in our TwoDArray
    pub fn searchForElement(&self, r:u32, c:u32) -> &T{
        let numRows = self.height;
        let numCols = self.width;
        // invariant in our code, this is the formula that will calculate the 1D index of a 2D array
        let newIndex: u32 = (numCols * r) + c;
        if r >= self.height || c >= self.width {
            eprintln!("there is something wrong");
        }
        return &self.matrix[newIndex as usize];
    }

    pub fn getRCbyIndex(&self, z:u32) -> (u32, u32){
        let c = z % self.width;
        let r = (z-c) / self.width;
        return (r, c);
    }

    // pub fn printMatrix(&self) {
    //     for i in 0..self.matrix.len() {
    //         print!("{}", self.matrix[i]);
    //     }
    // }

}

impl <T:Clone> TwoDArray<T>{
    /// Initializes a TwoDArray of nRows and nCols with a starting value
    pub fn initializeElements(startingValue: T, nRows: u32, nCols: u32) -> TwoDArray<T> {
        //! We could create this as a 1D array and use a function to add the same value to every element in the TwoDArray
        
        let mut new2D: TwoDArray<T> = TwoDArray { 
            matrix: Vec::new(), 
            height: nRows, 
            width: nCols };
        for _i in 0..nRows * nCols {
            let starting_value_clone = startingValue.clone();
            new2D.matrix.push(starting_value_clone);
        }
        return new2D;
    }
}