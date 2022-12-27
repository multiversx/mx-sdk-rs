import types


def vec_item(collection, item):
    return f"""
"str:{collection.name}.item|u32:{item.index}": "{item.value}","""


def set_item(collection, item):
    return f"""
"str:{collection.name}.node_links|u32:{item.index}": "u32:{item.prev}|u32:{item.next}",
"str:{collection.name}.node_id|{item.value}": "{item.index}",
"str:{collection.name}.value|u32:{item.index}": "{item.value}","""


def map_item(collection, item):
    return f"""{set_item(collection, item)}
"str:{collection.name}.mapped|{item.value}": "{item.mapped}","""


def vec_header(collection):
    return f'"str:{collection.name}.len": "{collection.length}",'


def list_header(collection):
    return f'"str:{collection.name}.info": "u32:{collection.length}|u32:{collection.first}|u32:{collection.last}|u32:{collection.next}",'


def format_collection(collection, items, format_header, format_item):
    collection_text = format_header(collection)
    item_texts = "".join([format_item(collection, item)
                          for item in items])
    return f'{collection_text}{item_texts}'


def make_collection(name, items):
    collection = types.SimpleNamespace()
    collection.name = name
    collection.length = len(items)
    collection.first = 1 if len(items) > 0 else 0
    collection.last = len(items)
    collection.next = collection.last
    return collection


def make_items(set_items):
    items = []
    for i, value in enumerate(set_items):
        item = types.SimpleNamespace()
        item.index = i + 1
        item.prev = i
        item.next = i + 2 if i < len(items) else 0
        item.value = value
        items.append(item)
    return items


def vec(name, items):
    """Generates the scenario code for a ``VecMapper`` storage.

    Examples
    --------
    >>> import format
    >>> print(format.vec("my_vec", ["str:hello", "str:foo"]))
    "str:my_vec.len": "2",
    "str:my_vec.item|u32:1": "str:hello",
    "str:my_vec.item|u32:2": "str:foo",
    """
    collection = make_collection(name, items)
    items = make_items(items)
    return format_collection(collection, items, vec_header, vec_item)


def set(name, items):
    """Generates the scenario code for a ``SetMapper`` storage.

    Examples
    --------
    >>> import format
    >>> print(format.set("my_vec", ["str:hello", "str:foo"]))
    "str:my_vec.info": "u32:2|u32:1|u32:2|u32:2",
    "str:my_vec.node_links|u32:1": "u32:0|u32:0",
    "str:my_vec.node_id|str:hello": "1",
    "str:my_vec.value|u32:1": "str:hello",
    "str:my_vec.node_links|u32:2": "u32:1|u32:0",
    "str:my_vec.node_id|str:foo": "2",
    "str:my_vec.value|u32:2": "str:foo",
    """
    collection = make_collection(name, items)
    items = make_items(items)
    return format_collection(collection, items, list_header, set_item)


def map(name, pair_list):
    """Generates the scenario code for a ``MapMapper`` storage.

    Examples
    --------
    >>> import format
    >>> print(format.map("my_map", [("str:hello", "str:world"), ("str:foo", "str:bar")]))
    "str:my_map.info": "u32:2|u32:1|u32:2|u32:2",
    "str:my_map.node_links|u32:1": "u32:0|u32:0",
    "str:my_map.node_id|str:hello": "1",
    "str:my_map.value|u32:1": "str:hello",
    "str:my_map.mapped|str:hello": "str:world",
    "str:my_map.node_links|u32:2": "u32:1|u32:0",
    "str:my_map.node_id|str:foo": "2",
    "str:my_map.value|u32:2": "str:foo",
    "str:my_map.mapped|str:foo": "str:bar",
    """
    keys = [key for key, value in pair_list]
    collection = make_collection(name, keys)
    items = make_items(keys)

    for item, (_, value) in zip(items, pair_list):
        item.mapped = value

    return format_collection(collection, items, list_header, map_item)


if __name__ == "__main__":
    import doctest
    doctest.testmod()
