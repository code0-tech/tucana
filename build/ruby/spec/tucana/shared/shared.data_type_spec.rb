# frozen_string_literal: true

Tucana.load_protocol(:shared)

RSpec.describe Tucana::Shared::DefinitionDataTypeRule do
  describe '#create' do
    context 'with :number_range variant' do
      it 'sets the number_range field' do
        config = { from: 1, to: 10 }
        rule = described_class.create(:number_range, config)
        expect(rule.number_range).to be_a(Tucana::Shared::DataTypeNumberRangeRuleConfig)
      end
    end

    context 'with :regex variant' do
      it 'sets the regex field' do
        config = { pattern: '\\d+' }
        rule = described_class.create(:regex, config)
        expect(rule.regex).to be_a(Tucana::Shared::DataTypeRegexRuleConfig)
      end
    end

    context 'with unknown variant' do
      it 'raises UnexpectedRuleType error' do
        expect do
          described_class.create(:unknown, {})
        end.to raise_error(Tucana::Shared::UnexpectedRuleType, 'Unknown rule type unknown')
      end
    end
  end
end
