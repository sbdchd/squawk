
class Squawk < Formula
  desc ""
  homepage ""
  url "https://github.com/sbdchd/squawk/archive/0.1.4.tar.gz"
  sha256 :no_check

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release"
    bin.install "target/release/squawk"
  end

  test do
    system "squawk", "--help"
  end
end
