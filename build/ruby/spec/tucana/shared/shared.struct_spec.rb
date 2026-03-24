# frozen_string_literal: true

Tucana.load_protocol(:shared)

RSpec.describe Tucana::Shared do
  describe "Struct" do
    subject(:struct) { Tucana::Shared::Struct.new }

    describe "#[]" do
      it "returns the ruby value for an existing key" do
        struct["name"] = "test"
        expect(struct["name"]).to eq("test")
      end

      it "returns nil for a missing key" do
        expect(struct["missing"]).to be_nil
      end
    end

    describe "#[]=" do
      it "sets string values" do
        struct["key"] = "value"
        expect(struct["key"]).to eq("value")
      end

      it "sets numeric values" do
        struct["num"] = 42
        expect(struct["num"]).to be_a(Tucana::Shared::NumberValue)
        expect(struct["num"].to_ruby).to eq(42)
      end

      it "sets boolean values" do
        struct["flag"] = true
        expect(struct["flag"]).to eq(true)
      end

      it "sets nil values" do
        struct["empty"] = nil
        expect(struct["empty"]).to be_nil
      end

      it "raises UnexpectedStructType for non-string keys" do
        expect { struct[123] = "val" }.to raise_error(Tucana::Shared::UnexpectedStructType)
      end
    end

    describe "#to_h" do
      it "returns a hash with ruby values" do
        struct["a"] = "hello"
        struct["b"] = 1
        expect(struct.to_h).to eq({ "a" => "hello", "b" => 1 })
      end
    end

    describe ".from_hash" do
      it "creates a Struct from a hash" do
        result = Tucana::Shared::Struct.from_hash({ "x" => "y", "n" => 5 })
        expect(result).to be_a(Tucana::Shared::Struct)
        expect(result.to_h).to eq({ "x" => "y", "n" => 5 })
      end
    end

    describe "#has_key?" do
      it "returns true for existing keys" do
        struct["key"] = "val"
        expect(struct.has_key?("key")).to be true
      end

      it "returns false for missing keys" do
        expect(struct.has_key?("nope")).to be false
      end
    end
  end

  describe "Value" do
    describe "#to_ruby" do
      it "converts string values" do
        val = Tucana::Shared::Value.from_ruby("hello")
        expect(val.to_ruby).to eq("hello")
      end

      it "converts boolean values" do
        val = Tucana::Shared::Value.from_ruby(true)
        expect(val.to_ruby).to eq(true)
      end

      it "converts nil" do
        val = Tucana::Shared::Value.from_ruby(nil)
        expect(val.to_ruby).to be_nil
      end

      it "returns Struct for struct values when not recursive" do
        val = Tucana::Shared::Value.from_ruby({ "a" => 1 })
        expect(val.to_ruby).to be_a(Tucana::Shared::Struct)
      end

      it "returns Hash for struct values when recursive" do
        val = Tucana::Shared::Value.from_ruby({ "a" => 1 })
        expect(val.to_ruby(true)).to eq({ "a" => 1 })
      end

      it "returns ListValue for list values when not recursive" do
        val = Tucana::Shared::Value.from_ruby([1, 2])
        expect(val.to_ruby).to be_a(Tucana::Shared::ListValue)
      end

      it "returns Array for list values when recursive" do
        val = Tucana::Shared::Value.from_ruby([1, 2])
        expect(val.to_ruby(true)).to eq([1, 2])
      end

      it "returns NumberValue for number values when not recursive" do
        val = Tucana::Shared::Value.from_ruby(42)
        expect(val.to_ruby).to be_a(Tucana::Shared::NumberValue)
      end

      it "returns ruby number for number values when recursive" do
        val = Tucana::Shared::Value.from_ruby(42)
        expect(val.to_ruby(true)).to eq(42)
      end
    end

    describe "#from_ruby" do
      it "handles Hash by converting to Struct" do
        val = Tucana::Shared::Value.from_ruby({ "k" => "v" })
        expect(val.kind).to eq(:struct_value)
      end

      it "handles Array by converting to ListValue" do
        val = Tucana::Shared::Value.from_ruby([1])
        expect(val.kind).to eq(:list_value)
      end

      it "handles Struct directly" do
        s = Tucana::Shared::Struct.from_hash({ "a" => 1 })
        val = Tucana::Shared::Value.from_ruby(s)
        expect(val.kind).to eq(:struct_value)
      end

      it "handles ListValue directly" do
        lv = Tucana::Shared::ListValue.from_a([1])
        val = Tucana::Shared::Value.from_ruby(lv)
        expect(val.kind).to eq(:list_value)
      end

      it "raises UnexpectedStructType for unsupported types" do
        expect { Tucana::Shared::Value.from_ruby(:symbol) }.to raise_error(Tucana::Shared::UnexpectedStructType)
      end
    end
  end

  describe "NumberValue" do
    describe "#to_ruby" do
      it "returns integer for integer values" do
        nv = Tucana::Shared::NumberValue.from_ruby(5)
        expect(nv.number).to eq(:integer)
        expect(nv.to_ruby).to eq(5)
        expect(nv.to_ruby).to be_a(Integer)
      end

      it "returns float for float values" do
        nv = Tucana::Shared::NumberValue.from_ruby(3.14)
        expect(nv.number).to eq(:float)
        expect(nv.to_ruby).to eq(3.14)
        expect(nv.to_ruby).to be_a(Float)
      end
    end

    describe "#from_ruby" do
      it "raises UnexpectedStructType for non-numeric types" do
        expect { Tucana::Shared::NumberValue.from_ruby("5") }.to raise_error(Tucana::Shared::UnexpectedStructType)
      end
    end
  end

  describe "ListValue" do
    subject(:list) { Tucana::Shared::ListValue.from_a([1, "two", true]) }

    describe "#length" do
      it "returns the number of elements" do
        expect(list.length).to eq(3)
      end
    end

    describe "#[]" do
      it "returns the ruby value at the index" do
        expect(list[1]).to eq("two")
      end
    end

    describe "#<<" do
      it "appends a value" do
        list << "new"
        expect(list.length).to eq(4)
        expect(list[3]).to eq("new")
      end
    end

    describe "#each" do
      it "yields ruby values" do
        values = []
        list.each { |v| values << v }
        expect(values).to include("two", true)
      end
    end

    describe "#to_a" do
      it "returns a ruby array" do
        expect(list.to_a).to eq([1, "two", true])
      end
    end

    describe ".from_a" do
      it "creates a ListValue from an array" do
        result = Tucana::Shared::ListValue.from_a(["a", 1])
        expect(result).to be_a(Tucana::Shared::ListValue)
        expect(result.to_a).to eq(["a", 1])
      end
    end

    describe "Enumerable" do
      it "supports Enumerable methods" do
        expect(list.map { |v| v }).to include("two")
      end
    end
  end
end
