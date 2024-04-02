#!/usr/bin/env python

import sys
from collections import Counter

import morfessor.baseline
from morfessor.io import MorfessorIO

# protoc ../src/morfessor/morfessor.proto --proto_path=../src/morfessor/ --python_out=. --pyi_out=.
import morfessor_pb2


def convert_annotation(annotation, annotation_pb):
    if not isinstance(annotation, dict):
        raise TypeError('annotation must be instance of list')

    if not isinstance(annotation_pb, morfessor_pb2.Annotation):
        raise TypeError('annotation_pb must be instance of morfessor_pb2.Annotation')

    for analysis in annotation:
        if not isinstance(analysis, list):
            raise TypeError('analysis must be instance of list')

        analysis_pb = morfessor_pb2.Analysis()

        analysis_pb.constructions.extend(analysis)

        annotation_pb.analyses.append(analysis_pb)


def convert_constr_node(constr_node, constr_node_pb):
    if not isinstance(constr_node, morfessor.baseline.ConstrNode):
        raise TypeError('constr_node must be instance of morfessor.baseline.ConstrNode')

    if not isinstance(constr_node_pb, morfessor_pb2.ConstrNode):
        raise TypeError('constr_node_pb must be instance of morfessor_pb2.ConstrNode')

    constr_node_pb.rcount = constr_node.rcount
    constr_node_pb.count = constr_node.count

    if type(constr_node.splitloc) is int:
        constr_node_pb.splitloc.append(constr_node.splitloc)

    if type(constr_node.splitloc) is tuple:
        constr_node_pb.splitloc.extend(constr_node.splitloc)


def convert_fixed_corpus_weight(corpus_weight_updater, corpus_weight_updater_pb):
    if not isinstance(corpus_weight_updater, morfessor.baseline.FixedCorpusWeight):
        raise TypeError('corpus_weight_updater must be instance of morfessor.baseline.FixedCorpusWeight')

    if not isinstance(corpus_weight_updater_pb, morfessor_pb2.FixedCorusWeight):
        raise TypeError('corpus_weight_updater_pb must be instance of morfessor_pb2.FixedCorusWeight')

    corpus_weight_updater_pb.weight = corpus_weight_updater.weight


def convert_counter(counter, counter_pb):
    if not isinstance(counter, Counter):
        raise TypeError('counter must be instance of Counter')

    if not isinstance(counter_pb, morfessor_pb2.Counter):
        raise TypeError('counter_pb must be instance of morfessor_pb2.Counter')

    for key, value in counter.items():
        counter_pb.counts[key] = value


def convert_encoding(encoding, encoding_pb):
    if not isinstance(encoding, morfessor.baseline.Encoding):
        raise TypeError('encoding must be instance of morfessor.baseline.Encoding')

    encoding_pb.logtokensum = encoding.logtokensum
    encoding_pb.tokens = encoding.tokens
    encoding_pb.boundaries = encoding.boundaries
    encoding_pb.weight = encoding.weight
    encoding_pb._log2pi = encoding._log2pi


def covert_lexicon_encoding(lexicon_encoding, lexicon_encoding_pb):
    if not isinstance(lexicon_encoding, morfessor.baseline.LexiconEncoding):
        raise TypeError('lexicon_encoding must be instance of morfessor.baseline.LexiconEncoding')

    if not isinstance(lexicon_encoding_pb, morfessor_pb2.LexiconEncoding):
        raise TypeError('lexicon_encoding_pb must be instance of morfessor_pb2.LexiconEncoding')

    convert_encoding(lexicon_encoding, lexicon_encoding_pb)

    atoms = morfessor_pb2.Counter()  # TODO necessary?

    convert_counter(lexicon_encoding.atoms, atoms)

    lexicon_encoding_pb.atoms.CopyFrom(atoms)


def convert_corpus_encoding(corpus_encoding, corpus_encoding_pb):
    if not isinstance(corpus_encoding, morfessor.baseline.CorpusEncoding):
        raise TypeError('corpus_encoding must be instance of morfessor.baseline.CorpusEncoding')

    if not isinstance(corpus_encoding_pb, morfessor_pb2.CorpusEncoding):
        raise TypeError('corpus_encoding_pb must be instance of morfessor_pb2.CorpusEncoding')

    convert_encoding(corpus_encoding, corpus_encoding_pb)

    lexicon_encoding_pb = morfessor_pb2.LexiconEncoding()  # TODO necessary?

    covert_lexicon_encoding(corpus_encoding.lexicon_encoding, lexicon_encoding_pb)

    corpus_encoding_pb.lexicon_encoding.CopyFrom(lexicon_encoding_pb)


def convert_annotated_corpus_encoding(annotated_corpus_encoding, annotated_corpus_encoding_pb):
    if not isinstance(annotated_corpus_encoding, morfessor.baseline.AnnotatedCorpusEncoding):
        raise TypeError('annotated_corpus_encoding must be instance of morfessor.baseline.AnnotatedCorpusEncoding')

    convert_encoding(annotated_corpus_encoding, annotated_corpus_encoding_pb)

    annotated_corpus_encoding_pb.do_update_weight = annotated_corpus_encoding.do_update_weight

    corpus_coding = morfessor_pb2.CorpusEncoding()  # TODO necessary?

    convert_corpus_encoding(corpus_coding, annotated_corpus_encoding.corpus_coding)

    annotated_corpus_encoding_pb.corpus_coding.CopyFrom(corpus_coding)


def main(argv):
    io = MorfessorIO()

    model = io.read_binary_model_file('unsup_model.bin')

    model_proto = morfessor_pb2.BaselineModel()

    if model.annotations:
        for compound, annotation in model.annotations.items():
            annotation_pb = morfessor_pb2.Annotation()

            convert_annotation(annotation, annotation_pb)

            model_proto.annotations[compound].CopyFrom(annotation_pb)

    if model.forcesplit_list:
        model_proto.forcesplit_list.extend(model.forcesplit_list)

    if model.nosplit_re:
        model_proto.nosplit_re = model.nosplit_re

    model_proto.penalty = model.penalty
    model_proto.tokens = model.tokens
    model_proto.types = model.types

    if model._analyses:
        for construction, node in model._analyses.items():
            node_pb = morfessor_pb2.ConstrNode()

            convert_constr_node(node, node_pb)

            model_proto._analyses[construction].CopyFrom(node_pb)

    if model._annot_coding:
        coding = morfessor_pb2.AnnotatedCorpusEncoding()

        convert_annotated_corpus_encoding(model._annot_coding, coding)

        model_proto._annot_coding.CopyFrom(coding)

    if model._corpus_coding:
        coding = morfessor_pb2.CorpusEncoding()

        convert_corpus_encoding(model._corpus_coding, coding)

        model_proto._corpus_coding.CopyFrom(coding)


    if model._corpus_weight_updater:
        updater = morfessor_pb2.FixedCorusWeight()

        convert_fixed_corpus_weight(model._corpus_weight_updater, updater)

        model_proto._corpus_weight_updater.CopyFrom(updater)

    if model._counter:
        counter = morfessor_pb2.Counter()

        convert_counter(model._counter, counter)

        # for construction, node in model._counter.items():
        #     counter.counts[construction] = node

        model_proto._counter.CopyFrom(counter)

    if model._lexicon_coding:
        coding = morfessor_pb2.LexiconEncoding()

        covert_lexicon_encoding(model._lexicon_coding, coding)

        model_proto._lexicon_coding.CopyFrom(coding)

    model_proto._segment_only = model._segment_only
    model_proto._supervised = model._supervised
    model_proto._use_skips = model._use_skips

    # write to file
    with open('unsup_model.proto', 'wb') as f:
        f.write(model_proto.SerializeToString())

    print('lol')


if __name__ == "__main__":
    main(sys.argv[1:])
