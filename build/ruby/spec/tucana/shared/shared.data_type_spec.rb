# frozen_string_literal: true

Tucana.load_protocol(:shared)

RSpec.describe Tucana::Shared::DefinitionDataTypeRule do
  describe "#create" do
    context 'DataTypeIdentifier#to_h' do
      context 'generic_key' do
        it do
          expect(Tucana::Shared::DataTypeIdentifier.new(generic_key: 'T').to_h)
            .to eq({ generic_key: 'T' })
        end
      end
      context 'identifier' do
        it do
          expect(Tucana::Shared::DataTypeIdentifier.new(data_type_identifier: 'MyType').to_h)
            .to eq({ data_type_identifier: 'MyType' })
        end
      end
      context 'generic_type' do
        it do
          generic_type = Tucana::Shared::GenericType.new(
            data_type_identifier: 'some_identifier',
            generic_mappers: []
          )

          data_type_identifier = Tucana::Shared::DataTypeIdentifier.new(generic_type: generic_type)

          expect(data_type_identifier.to_h)
            .to eq({
                     generic_type: {
                       data_type_identifier: 'some_identifier',
                       generic_mappers: []
                     }
                   })
        end
      end
    end

    context "with :contains_key variant" do
      it "sets the contains_key field" do
        config = { key: "test_key", data_type_identifier: { data_type_identifier: "test_type" } }
        rule = described_class.create(:contains_key, config)
        expect(rule.contains_key).to be_a(Tucana::Shared::DefinitionDataTypeContainsKeyRuleConfig)
      end
    end

    context "with :contains_type variant" do
      it "sets the contains_type field" do
        config = { data_type_identifier: { data_type_identifier: "test_type" } }
        rule = described_class.create(:contains_type, config)
        expect(rule.contains_type).to be_a(Tucana::Shared::DefinitionDataTypeContainsTypeRuleConfig)
      end
    end

    context "with :item_of_collection variant" do
      it "sets the item_of_collection field" do
        config = { items: %w[item1 item2] }
        rule = described_class.create(:item_of_collection, config)
        expect(rule.item_of_collection).to be_a(Tucana::Shared::DataTypeItemOfCollectionRuleConfig)
      end
    end

    context "with :number_range variant" do
      it "sets the number_range field" do
        config = { from: 1, to: 10 }
        rule = described_class.create(:number_range, config)
        expect(rule.number_range).to be_a(Tucana::Shared::DataTypeNumberRangeRuleConfig)
      end
    end

    context "with :regex variant" do
      it "sets the regex field" do
        config = { pattern: "\\d+" }
        rule = described_class.create(:regex, config)
        expect(rule.regex).to be_a(Tucana::Shared::DataTypeRegexRuleConfig)
      end
    end

    context "with :input_types variant" do
      it "sets the input_types field" do
        config = { input_types: [{ data_type_identifier: { data_type_identifier: "test_type" }, input_identifier: "test_input" }] }
        rule = described_class.create(:input_types, config)
        expect(rule.input_types).to be_a(Tucana::Shared::DefinitionDataTypeInputTypesRuleConfig)
      end
    end

    context "with :return_type variant" do
      it "sets the return_type field" do
        config = { data_type_identifier: { data_type_identifier: "test_type" } }
        rule = described_class.create(:return_type, config)
        expect(rule.return_type).to be_a(Tucana::Shared::DefinitionDataTypeReturnTypeRuleConfig)
      end
    end

    context "with :parent_type variant" do
      it "sets the parent_type field" do
        config = { parent_type: { data_type_identifier: "test_type" } }
        rule = described_class.create(:parent_type, config)
        expect(rule.parent_type).to be_a(Tucana::Shared::DefinitionDataTypeParentTypeRuleConfig)
      end
    end

    context "with unknown variant" do
      it "raises UnexpectedRuleType error" do
        expect {
          described_class.create(:unknown, {})
        }.to raise_error(Tucana::Shared::UnexpectedRuleType, "Unknown rule type unknown")
      end
    end
  end
end
