# Build tool for uwu

require 'open3'
require 'optparse'

# Utils for executing OS commands
def exec_cmd(str, chdir)
	puts "$ #{str.cyan.italic}"
	Open3.popen3(str, :chdir=>chdir) do |stdin, stdout, stderr, wait_thr|
	  exit_status = wait_thr.value
	  unless exit_status.success?
	    abort "Fail".red
	  end
	end
end

def exec_with_timeout(cmd, timeout)
  pid = Process.spawn(cmd, {[:err,:out] => :close, :pgroup => true})
  begin
    Timeout.timeout(timeout) do
      Process.waitpid(pid, 0)
      $?.exitstatus == 0
    end
  rescue Timeout::Error
    Process.kill(15, -Process.getpgid(pid))
    false
  end
end

def exec_bash(str)
	puts "$ bash -c #{str.cyan.italic}"
	Open3.popen3("bash", "-c", str) do |stdin, stdout, stderr, wait_thr|
	  exit_status = wait_thr.value
	  unless exit_status.success?
	    abort "Fail".red
	  end
	 end
end

# String stylizing
class String
	def black;          "\e[30m#{self}\e[0m" end
	def red;            "\e[31m#{self}\e[0m" end
	def green;          "\e[32m#{self}\e[0m" end
	def brown;          "\e[33m#{self}\e[0m" end
	def blue;           "\e[34m#{self}\e[0m" end
	def magenta;        "\e[35m#{self}\e[0m" end
	def cyan;           "\e[36m#{self}\e[0m" end
	def gray;           "\e[37m#{self}\e[0m" end

	def bg_black;       "\e[40m#{self}\e[0m" end
	def bg_red;         "\e[41m#{self}\e[0m" end
	def bg_green;       "\e[42m#{self}\e[0m" end
	def bg_brown;       "\e[43m#{self}\e[0m" end
	def bg_blue;        "\e[44m#{self}\e[0m" end
	def bg_magenta;     "\e[45m#{self}\e[0m" end
	def bg_cyan;        "\e[46m#{self}\e[0m" end
	def bg_gray;        "\e[47m#{self}\e[0m" end

	def bold;           "\e[1m#{self}\e[22m" end
	def italic;         "\e[3m#{self}\e[23m" end
	def underline;      "\e[4m#{self}\e[24m" end
	def blink;          "\e[5m#{self}\e[25m" end
	def reverse_color;  "\e[7m#{self}\e[27m" end
end

options = {}

subtext = <<HELP
Commonly used command are:
   wasm :     build wasm
   cli  :     build CLI
   test :     run `cargo test`
See 'x.rb COMMAND --help' for more information on a specific command.
HELP

global = OptionParser.new do |opts|
  opts.banner = "Usage: x.rb [options] [subcommand [options]]"
  opts.on("-v", "--[no-]verbose", "Run verbosely") do |v|
    options[:verbose] = v
  end
  opts.separator ""
  opts.separator subtext
end

subcommands = { 
  'wasm' => OptionParser.new do |opts|
      opts.banner = "Usage: wasm [options]"
      opts.on("-d", "--debug", "Build debug builds") do |v|
        options[:debug] = v
      end
   end,
   'cli' => OptionParser.new do |opts|
        opts.banner = "Usage: cli [options]"
        opts.on("-d", "--debug", "Build debug builds") do |v|
          options[:debug] = v
        end
    end,
    'test' => OptionParser.new do |opts|
      opts.banner = "Usage: test [options]"
    end,
 }

global.order!
command = ARGV.shift
unless command
  STDERR.puts "ERROR: no subcommand"
  STDERR.puts global # prints usage
  exit(-1)
end
subcommands[command].order!

# trigger wasm builds
def build_wasm
    exec_cmd("wasm-pack build --target nodejs --release -d pkg-node", "wasm")
    exec_cmd("wasm-pack build --target web --release -d pkg", "wasm")
end

def build_cli
    exec_cmd("cargo build", ".")
end

def run_tests
  exec_cmd("cargo test", ".")
end

case command
when "wasm"
  build_wasm()
when "cli"
  build_cli()
when "test"
  run_tests()
else 
  STDERR.puts "ERROR: no subcommand"
  STDERR.puts global # prints usage
end

