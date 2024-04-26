import pyscryer


def test_engine_works():
    script = """
    man("Mark").
    child("John", "Mark").
    father(F, C) :- child(C, F), man(F).
    """.strip()

    query = 'father(F, "John").'

    engine = pyscryer.PrologEngine()

    engine.load_module("test", script)
    print("loaded module")
    answer = engine.run_query(query)
    print(f"running query: {query}")
    assert answer == [{'F': 'Mark'}]
