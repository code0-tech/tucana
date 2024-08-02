# frozen_string_literal: true

require_relative "tucana/version"

module Tucana
  class Error < StandardError; end

  def self.load_protocol(protocol)
    this_dir = File.expand_path(File.dirname(__FILE__))
    protocol_dir = File.join(this_dir, "tucana/#{protocol}")
    $LOAD_PATH.unshift(protocol_dir) unless $LOAD_PATH.include?(protocol_dir)

    Dir["#{protocol_dir}/*_pb.rb"].each { |file| require file }
  end
end
