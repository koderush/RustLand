# RustLand
A Rust playground.

# Tool Chain
`sudo apt update`

`sudo apt install gcc`

`sudo apt install pkg-config`

`sudo apt install libssl-dev`

# Run
`cargo run`

# Debug
Need to install `CodeLLDB`

## CentOS 7 Missing GLIBC_2.18
### Solution from (https://github.com/vadimcn/vscode-lldb/issues/268)
1. Build glibc-2.18
```
cd /opt/
wget http://ftp.gnu.org/gnu/glibc/glibc-2.18.tar.gz
tar zxvf glibc-2.18.tar.gz
cd glibc-2.18
mkdir build
cd build
../configure --prefix=/opt/glibc-2.18
make -j4
sudo make install
```
2. May need to install `patchelf`:
```
sudo yum install patchelf
```
3. Patch the CodeLLDB
```
[kyou@kyou-dev-c7]$ patchelf --set-interpreter /opt/glibc-2.18/lib/ld-linux-x86-64.so.2 ~/.vscode/extensions/vadimcn.vscode-lldb-1.6.1/adapter/codelldb
[kyou@kyou-dev-c7]$ patchelf --set-interpreter /opt/glibc-2.18/lib/ld-linux-x86-64.so.2 ~/.vscode/extensions/vadimcn.vscode-lldb-1.6.1/lldb/bin/lldb-server 
[kyou@kyou-dev-c7]$ patchelf --set-interpreter /opt/glibc-2.18/lib/ld-linux-x86-64.so.2 ~/.vscode/extensions/vadimcn.vscode-lldb-1.6.1/lldb/bin/lldb
```
4. Add the below content to `~/.bashrc`
```
LD_PRELOAD=/usr/lib64/libgcc_s.so.1
```
