# frozen_string_literal: true

# Protocol Buffers - Google's data interchange format
# Copyright 2008 Google Inc.  All rights reserved.
#
# Use of this source code is governed by a BSD-style
# license that can be found at https://github.com/protocolbuffers/protobuf/blob/080a983eaaec3c57178c604738d245c70d9059ba/LICENSE or at
# https://developers.google.com/open-source/licenses/bsd

# Code0 Tucana note:
# This file only contains a partial copy of the source file. The module names have been modified to accommodate for the needs of this project.
# The original is located at https://github.com/protocolbuffers/protobuf/blob/18aa4e1fcb8882133e96445c40d1521f16932f1c/ruby/lib/google/protobuf/well_known_types.rb

module Tucana
  module Shared
    UnexpectedStructType = Class.new(Tucana::Error)

    class Struct
      def [](key)
        self.fields[key].to_ruby
      rescue NoMethodError
        nil
      end

      def []=(key, value)
        unless key.is_a?(String)
          raise UnexpectedStructType, "Struct keys must be strings."
        end
        self.fields[key] ||= Value.new
        self.fields[key].from_ruby(value)
      end

      def to_h
        ret = {}
        self.fields.each { |key, val| ret[key] = val.to_ruby(true) }
        ret
      end

      def self.from_hash(hash)
        ret = Struct.new
        hash.each { |key, val| ret[key] = val }
        ret
      end

      def has_key?(key)
        self.fields.has_key?(key)
      end
    end

    class Value
      def to_ruby(recursive = false)
        case self.kind
        when :struct_value
          if recursive
            self.struct_value.to_h
          else
            self.struct_value
          end
        when :list_value
          if recursive
            self.list_value.to_a
          else
            self.list_value
          end
        when :null_value
          nil
        when :number_value
          self.number_value
        when :string_value
          self.string_value
        when :bool_value
          self.bool_value
        else
          raise UnexpectedStructType
        end
      end

      def self.from_ruby(value)
        self.new.from_ruby(value)
      end

      def from_ruby(value)
        case value
        when NilClass
          self.null_value = :NULL_VALUE
        when Numeric
          self.number_value = value
        when String
          self.string_value = value
        when TrueClass
          self.bool_value = true
        when FalseClass
          self.bool_value = false
        when Struct
          self.struct_value = value
        when Hash
          self.struct_value = Struct.from_hash(value)
        when ListValue
          self.list_value = value
        when Array
          self.list_value = ListValue.from_a(value)
        else
          raise UnexpectedStructType
        end

        self
      end
    end

    class ListValue
      include Enumerable

      def length
        self.values.length
      end

      def [](index)
        self.values[index].to_ruby
      end

      def []=(index, value)
        self.values[index].from_ruby(value)
      end

      def <<(value)
        wrapper = Value.new
        wrapper.from_ruby(value)
        self.values << wrapper
      end

      def each
        self.values.each { |x| yield(x.to_ruby) }
      end

      def to_a
        self.values.map { |x| x.to_ruby(true) }
      end

      def self.from_a(arr)
        ret = ListValue.new
        arr.each { |val| ret << val }
        ret
      end
    end
  end
end
