# frozen_string_literal: true

module Tucana
  module Shared
    UnexpectedRuleType = Class.new(Tucana::Error)
    UnexpectedType = Class.new(Tucana::Error)

    DefinitionDataTypeRule.class_eval do
      def variant
        self.config
      end

      def rule_config
        case variant
        when :number_range
          self.number_range
        when :regex
          self.regex
        else
          raise UnexpectedRuleType, "Unknown rule type #{variant}"
        end
      end

      def create(variant, config)
        case variant
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
        new.create(variant, config)
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
  end
end
