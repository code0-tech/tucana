# frozen_string_literal: true

module Tucana
  module Shared
    UnexpectedRuleType = Class.new(Tucana::Error)

    DataTypeRule.class_eval do
      def variant
        self.config
      end

      def create(variant, config)
        case variant
        when :contains_key
          self.contains_key = DataTypeContainsKeyRuleConfig.new(config)
        when :contains_type
          self.contains_type = DataTypeContainsTypeRuleConfig.new(config)
        when :item_of_collection
          self.item_of_collection = DataTypeItemOfCollectionRuleConfig.new(config)
        when :number_range
          self.number_range = DataTypeNumberRangeRuleConfig.new(config)
        when :regex
          self.regex = DataTypeRegexRuleConfig.new(config)
        else
          raise UnexpectedRuleType, "Unknown rule type #{variant}"
        end

        self
      end

      def self.create(variant, config)
        self.new.create(variant, config)
      end
    end
  end
end
