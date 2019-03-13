fn main()
{
    // get option from command line args with clap
    option = get_options();
    
    // open and read file. Check if file is not empty
    raw_content = get_raw_content(option.file);

    // return a new String without comments
    filtered_content = filter_comments(raw_content);

    // get puzzle size (first number in file)
    n = get_puzzle_size(filtered_content);

    // remove size from content
    filtered_content = filtered_content[1...];

    // get vector containing 


}