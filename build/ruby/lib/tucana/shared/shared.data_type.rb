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
        when :parent_type
          self.parent_type
        else
          raise UnexpectedRuleType, "Unknown rule type #{variant}"
        end
      end

      def create(variant, config)
        case variant
        when :contains_key
          self.contains_key = DefinitionDataTypeContainsKeyRuleConfig.new(config)
        when :contains_type
          self.contains_type = DefinitionDataTypeContainsTypeRuleConfig.new(config)
        when :item_of_collection
          self.item_of_collection = DataTypeItemOfCollectionRuleConfig.from_hash(config)
        when :number_range
          self.number_range = DataTypeNumberRangeRuleConfig.new(config)
        when :regex
          self.regex = DataTypeRegexRuleConfig.new(config)
        when :input_types
          self.input_types = DefinitionDataTypeInputTypesRuleConfig.new(config)
        when :return_type
          self.return_type = DefinitionDataTypeReturnTypeRuleConfig.new(config)
        when :parent_type
          self.parent_type = DefinitionDataTypeParentTypeRuleConfig.new(config)
        else
          raise UnexpectedRuleType, "Unknown rule type #{variant}"
        end

        self
      end

      def self.create(variant, config)
        new.create(variant, config)
      end
    end

    DefinitionDataTypeContainsKeyRuleConfig.class_eval do
      def to_h
        {
          key: self.key,
          data_type_identifier: self.data_type_identifier.to_h,
        }
      end

      def self.from_hash(config)
        new(
          key: config[:key],
          data_type_identifier: DataTypeIdentifier.from_hash(config[:data_type_identifier])
        )
      end
    end

    DefinitionDataTypeContainsTypeRuleConfig.class_eval do
      def to_h
        {
          data_type_identifier: self.data_type_identifier.to_h,
        }
      end

      def self.from_hash(config)
        new(data_type_identifier: DataTypeIdentifier.from_hash(config[:data_type_identifier]))
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

    DefinitionDataTypeInputTypesRuleConfig.class_eval do
      def to_h
        {
          input_types: self.input_types.map { |input_type| input_type.to_h }
        }
      end

      def self.from_hash(hash)
        new(
          input_types: hash.fetch(:input_types).map { |input_type| DataTypeInputType.from_hash(input_type) }
        )
      end
    end

    DefinitionDataTypeInputTypesRuleConfig::DataTypeInputType.class_eval do
      def to_h
        {
          data_type_identifier: self.data_type_identifier,
          input_identifier: self.input_identifier,
        }
      end

      def self.from_hash(config)
        new(
          input_identifier: config[:input_identifier],
          data_type_identifier: DataTypeIdentifier.from_hash(config[:data_type_identifier])
        )
      end
    end

    DefinitionDataTypeReturnTypeRuleConfig.class_eval do
      def to_h
        {
          data_type_identifier: self.data_type_identifier,
        }
      end

      def self.from_hash(config)
        new(data_type_identifier: DataTypeIdentifier.from_hash(config[:data_type_identifier]))
      end
    end

    DefinitionDataTypeParentTypeRuleConfig.class_eval do
      def to_h
        {
          parent_type: self.parent_type,
        }
      end

      def self.from_hash(config)
        new(parent_type: DataTypeIdentifier.from_hash(config[:parent_type]))
      end
    end

    DataTypeIdentifier.class_eval do
      def to_h
        {
          data_type_identifier: self.data_type_identifier,
          generic_type: self.generic_type ? self.generic_type.to_h : nil,
          generic_key: self.generic_key,
        }
      end

      def from_hash(config)
        if config.keys.intersection([:data_type_identifier, :generic_type, :generic_key]).count > 1
          raise UnexpectedType, "Cannot have more than one type"
        end

        if config.key?(:data_type_identifier)
          self.data_type_identifier = config
        elsif config.key?(:generic_type)
          self.generic_type = GenericType.new(config)
        elsif config.key?(:generic_key)
          self.generic_key = config
        else
          raise UnexpectedType, "Unknown type"
        end

        self
      end

      def self.from_hash(config)
        new.from_hash(config)
      end
    end

    GenericType.class_eval do
      def to_h
        {
          data_type_identifier: self.data_type_identifier.to_h,
          generic_mapper: self.generic_mapper ? self.generic_mapper.to_h : nil,
        }
      end

      def from_hash(config)
        if config.key?(:data_type_identifier)
          self.data_type_identifier = DataTypeIdentifier.from_hash(config.fetch(:data_type_identifier))
        end

        if config.key?(:generic_mapper)
          self.generic_mapper = GenericMapper.from_hash(config.fetch(:generic_mapper))
        end

        self
      end

      def self.from_hash(config)
        new.from_hash(config)
      end
    end

    GenericMapper.class_eval do
      def to_h
        {
          data_type_identifier: self.data_type_identifier.to_h,
          generic_key: self.generic_key,
          source: self.source,
        }
      end

      def from_hash(config)
        if config.keys.intersection([:data_type_identifier, :generic_key]).count > 1
          raise UnexpectedType, "Cannot have more than one type"
        end

        if config.key?(:data_type_identifier)
          self.data_type_identifier = DataTypeIdentifier.new(config.fetch(:data_type_identifier))
        elsif config.key?(:generic_key)
          self.generic_key = config.fetch(:generic_key)
        end

        self.source = config[:source]
      end

      def self.from_hash(config)
        new.from_hash(config)
      end
    end
  end
end
