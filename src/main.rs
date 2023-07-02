use std::path::Path;
use std::time::SystemTime;
use chrono::{DateTime, Local};

fn main()
{
    let cli_args: Vec<String> = std::env::args().collect();

    println!();

    for arg_idx in 1..cli_args.len()
    {
        let cli_arg = &cli_args[arg_idx];

        let possible_file_path = Path::new( cli_arg );

        println!( "                Path: {}", possible_file_path.display() );
        println!( "              Exists: {}", possible_file_path.exists() );
        println!( "    Path is absolute: {}", possible_file_path.is_absolute() );
        println!( "        Is directory: {}", possible_file_path.is_dir() );
        println!( "             Is file: {}", possible_file_path.is_file() );
        println!( "          Is symlink: {}", possible_file_path.is_symlink() );

        if let Ok( metadata_for_path ) = possible_file_path.metadata()
        {
            if let Ok( creation_time ) = metadata_for_path.created()
            {
                println!( "       Creation time: {}", systemtime_to_time_string( creation_time ) );
            }

            if let Ok( last_accessed_time ) = metadata_for_path.accessed()
            {
                println!( "  Last Accessed time: {}", systemtime_to_time_string( last_accessed_time ) );
            }

            if let Ok( modification_time ) = metadata_for_path.modified()
            {
                println!( "   Modification time: {}", systemtime_to_time_string( modification_time ) );
            }
        }

        println!();
    }
}

fn systemtime_to_time_string( timestamp_as_systemtime: SystemTime ) -> String
{
    let systemtime_as_datetime: DateTime<Local> = timestamp_as_systemtime.into();

    systemtime_as_datetime.format( "%H:%M:%S %a %d-%b-%Y" ).to_string()
}