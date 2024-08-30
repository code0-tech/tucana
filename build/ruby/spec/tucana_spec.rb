# frozen_string_literal: true

RSpec.describe Tucana do
  it "has a version number" do
    expect(Tucana::VERSION).not_to be_nil
  end

  it "can load internal protocol" do
    expect { Tucana.load_protocol(:internal) }.not_to raise_error
  end

  it "can load actions protocol" do
    expect { Tucana.load_protocol(:actions) }.not_to raise_error
  end
end
