class Sprout < Formula
  desc "Terminal-based habit tracker"
  homepage "https://github.com/kb019/sprout"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/kb019/sprout/releases/download/v#{version}/sprout-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER"
    end
    on_intel do
      url "https://github.com/kb019/sprout/releases/download/v#{version}/sprout-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/kb019/sprout/releases/download/v#{version}/sprout-v#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER"
    end
    on_intel do
      url "https://github.com/kb019/sprout/releases/download/v#{version}/sprout-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER"
    end
  end

  def install
    bin.install "sprout"
  end

  test do
    system "#{bin}/sprout", "--version"
  end
end
