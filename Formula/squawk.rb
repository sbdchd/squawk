
class Squawk < Formula
  desc ""
  homepage ""
  url "https://github.com/sbdchd/squawk/archive/0.1.0.tar.gz"
  sha256 "385a02eac30d931b525342ad3ffb21676c31c352bf336db02705db5db050dfbd"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release"
    bin.install "target/release/squawk"
  end

  test do
    system "squawk", "--help"
  end
end
