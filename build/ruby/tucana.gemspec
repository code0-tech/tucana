# frozen_string_literal: true

require_relative "lib/tucana/version"

Gem::Specification.new do |spec|
  spec.name = "tucana"
  spec.version = Tucana::VERSION
  spec.authors = ["Niklas van Schrick"]
  spec.email = ["mc.taucher2003@gmail.com"]

  spec.summary = "Code0 GRPC Protocol."
  spec.homepage = "https://github.com/code0-tech/tucana"
  spec.license = "Apache-2.0"
  spec.required_ruby_version = ">= 3.0.0"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = spec.homepage
  spec.metadata["changelog_uri"] = "#{spec.homepage}/releases"

  # Specify which files should be added to the gem when it is released.
  spec.files = Dir["lib/**/*.rb"]
  spec.require_paths = ["lib"]

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html

  spec.add_dependency "grpc", "~> 1.65"

  spec.add_development_dependency "grpc-tools", "~> 1.65"
  spec.add_development_dependency "rake", "~> 13.0"
  spec.add_development_dependency "rspec", "~> 3.0"
  spec.metadata["rubygems_mfa_required"] = "true"
end
