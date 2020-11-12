require 'yaml'

def require_field(hash, field_name, obj_name)
  if hash[field_name].nil?
    raise "#{obj_name} has no #{field_name}"
  end
end

def assert_node(node, idx)
  require_field(node, 'struct_name', "node at idx #{idx}")
  obj_name = "node #{node['struct_name']}"
  require_field(node, 'str_type', obj_name)
  require_field(node, 'filename', obj_name)
  require_field(node, 'comment', obj_name)
  require_field(node, 'fields', obj_name)

  assert_fields(node)
end

KNOWN_TYPES = %w[
  Node
  Nodes
  MaybeNode
  Range
  MaybeRange
  Str
  MaybeStr
  Chars
  StringValue
  U8
  Usize
  RawString
  RegexOptions
]

def assert_fields(node)
  node_prefix = "node #{node['struct_name']}"
  node['fields'].each_with_index do |field, idx|
    require_field(field, 'field_name', "#{node_prefix} -> field at idx #{idx}")
    obj_name = "#{node_prefix} -> field #{field['field_name']}"

    require_field(field, 'field_type', obj_name)
    require_field(field, 'always_print', obj_name)
    require_field(field, 'comment', obj_name)

    unless KNOWN_TYPES.include?(field['field_type'])
      raise "#{obj_name} -> field_type is unknown: #{field['field_type']}"
    end
  end
end

YAML.load_file('src/nodes.yaml').each_with_index do |node, idx|
  assert_node(node, idx)
end
