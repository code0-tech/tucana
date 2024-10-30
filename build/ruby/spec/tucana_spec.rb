# frozen_string_literal: true

RSpec.describe Tucana do
  it "has a version number" do
    expect(Tucana::VERSION).not_to be_nil
  end

  Tucana::AVAILABLE_PROTOCOLS.each do |protocol|
    it "can load #{protocol} protocol" do
      expect { Tucana.load_protocol(protocol) }.not_to raise_error
    end
  end
end
