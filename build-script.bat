cargo build --release

mkdir auto_clicker

robocopy target/release/ auto_clicker/ *.exe
robocopy resources auto_clicker/ 

