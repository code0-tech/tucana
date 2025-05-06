# frozen_string_literal: true

module Tucana
  module Shared
    UnexpectedRuleType = Class.new(Tucana::Error)

    DataTypeRule.class_eval do
      def variant
        self.config
      end

      def rule_config
        case variant
        when :contains_key
          self.contains_key
        when :contains_type
          self.contains_type
        when :item_of_collection
          self.item_of_collection
        when :number_range
          self.number_range
        when :regex
          self.regex
        when :input_types
          self.input_types
        when :return_type
          self.return_type
        else
          raise UnexpectedRuleType, "Unknown rule type #{variant}"
        end
      end

      def create(variant, config)
        case variant
        when :contains_key
          self.contains_key = DataTypeContainsKeyRuleConfig.new(config)
        when :contains_type
          self.contains_type = DataTypeContainsTypeRuleConfig.new(config)
        when :item_of_collection
          self.item_of_collection = DataTypeItemOfCollectionRuleConfig.from_hash(config)
        when :number_range
          self.number_range = DataTypeNumberRangeRuleConfig.new(config)
        when :regex
          self.regex = DataTypeRegexRuleConfig.new(config)
        when :input_types
          self.input_types = DataTypeInputTypesRuleConfig.new(config)
        when :return_type
          self.return_type = DataTypeReturnTypeRuleConfig.new(config)
        else
          raise UnexpectedRuleType, "Unknown rule type #{variant}"
        end

        self
      end

      def self.create(variant, config)
        self.new.create(variant, config)
      end
    end

    DataTypeContainsKeyRuleConfig.class_eval do
      def to_h
        {
          key: self.key,
          data_type_identifier: self.data_type_identifier,
        }
      end
    end

    DataTypeContainsTypeRuleConfig.class_eval do
      def to_h
        {
          data_type_identifier: self.data_type_identifier,
        }
      end
    end

    DataTypeItemOfCollectionRuleConfig.class_eval do
      def to_h
        {
          items: self.items.map { |item| item.to_ruby(true) },
        }
      end

      def self.from_hash(hash)
        new(items: hash.fetch(:items).map { |item| Value.from_ruby(item) })
      end
    end

    DataTypeNumberRangeRuleConfig.class_eval do
      def to_h
        {
          from: self.from,
          to: self.to,
          steps: self.steps,
        }
      end
    end

    DataTypeRegexRuleConfig.class_eval do
      def to_h
        {
          pattern: self.pattern,
        }
      end
    end

    DataTypeInputTypesRuleConfig.class_eval do
      def to_h
        {
          input_types: self.input_types.map { |input_type| input_type.to_h }
        }
      end
    end

    DataTypeInputTypesRuleConfig::DataTypeInputType.class_eval do
      def to_h
        {
          data_type_identifier: self.data_type_identifier,
          input_identifier: self.input_identifier,
        }
      end
    end

    DataTypeReturnTypeRuleConfig.class_eval do
      def to_h
        {
          data_type_identifier: self.data_type_identifier,
        }
      end
    end
  end
end
