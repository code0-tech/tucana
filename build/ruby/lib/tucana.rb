# frozen_string_literal: true

require_relative "tucana/version"

module Tucana
  class Error < StandardError; end

  AVAILABLE_PROTOCOLS = %i[aquila sagittarius]

  def self.load_protocol(protocol)
    if protocol != :shared
      load_protocol(:shared)
    end

    this_dir = File.expand_path(File.dirname(__FILE__))

    generated_protocol_dir = File.join(this_dir, "tucana/generated/#{protocol}")
    Dir["#{generated_protocol_dir}/*_pb.rb"].each { |file| require file }

    protocol_dir = File.join(this_dir, "tucana/#{protocol}")
    Dir["#{protocol_dir}/*.rb"].each { |file| require file }
  end
end
