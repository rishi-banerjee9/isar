import 'package:copy_with_extension/copy_with_extension.dart';
import 'package:isar/isar.dart';
import 'package:isar_test/isar_test.dart';
import 'package:test/test.dart';

part 'update_test.g.dart';

@collection
@CopyWith()
class Model {
  Model({
    required this.id,
    required this.boolProp,
    required this.nullableBoolProp,
    required this.byteProp,
    required this.shortProp,
    required this.longProp,
    required this.floatProp,
    required this.doubleProp,
    required this.stringProp,
    required this.nullableStringProp,
    required this.dateProp,
  });

  final int id;

  final bool boolProp;

  final bool? nullableBoolProp;

  final byte byteProp;

  final short shortProp;

  final int longProp;

  final float floatProp;

  final double doubleProp;

  final String stringProp;

  final String? nullableStringProp;

  final DateTime dateProp;

  @override
  // ignore: hash_and_equals, avoid_equals_and_hash_code_on_mutable_classes
  bool operator ==(Object other) =>
      other is Model &&
      other.id == id &&
      other.boolProp == boolProp &&
      other.byteProp == byteProp &&
      other.shortProp == shortProp &&
      other.longProp == longProp &&
      other.floatProp == floatProp &&
      other.doubleProp == doubleProp &&
      other.stringProp == stringProp &&
      other.nullableStringProp == nullableStringProp &&
      other.dateProp == dateProp;

  @override
  String toString() {
    // TODO: implement toString
    return 'Model(id: $id, boolProp: $boolProp, nullableBoolProp: $nullableBoolProp, byteProp: $byteProp, shortProp: $shortProp, longProp: $longProp, floatProp: $floatProp, doubleProp: $doubleProp, stringProp: $stringProp, dateProp: $dateProp, nullableStringProp: $nullableStringProp)';
  }
}

void main() {
  group('Update', () {
    late Isar isar;
    late Model model;

    setUp(() {
      isar = openTempIsar([ModelSchema]);

      model = Model(
        id: 12,
        boolProp: true,
        nullableBoolProp: false,
        byteProp: 1,
        shortProp: 2,
        longProp: 3,
        floatProp: 4,
        doubleProp: 5,
        stringProp: 'hello',
        nullableStringProp: 'world',
        dateProp:
            DateTime.fromMillisecondsSinceEpoch(200, isUtc: true).toLocal(),
      );
      isar.writeTxn((isar) => isar.models.put(model));
    });

    group('update()', () {
      isarTest('bool change', () {
        isar.writeTxn((isar) {
          expect(isar.models.update(model.id, boolProp: false), true);
        });
        expect(isar.models.get(model.id), model.copyWith(boolProp: false));
      });

      isarTest('bool change to null', () {
        isar.writeTxn((isar) {
          expect(isar.models.update(model.id, nullableBoolProp: null), true);
        });
        expect(
          isar.models.get(model.id),
          model.copyWith(nullableBoolProp: null),
        );
      });

      isarTest('byte change', () {
        isar.writeTxn((isar) {
          expect(isar.models.update(model.id, byteProp: 2), true);
        });
        expect(isar.models.get(model.id), model.copyWith(byteProp: 2));
      });

      isarTest('short change', () {
        isar.writeTxn((isar) {
          expect(isar.models.update(model.id, shortProp: 3), true);
        });
        expect(isar.models.get(model.id), model.copyWith(shortProp: 3));
      });

      isarTest('long change', () {
        isar.writeTxn((isar) {
          expect(isar.models.update(model.id, longProp: 4), true);
        });
        expect(isar.models.get(model.id), model.copyWith(longProp: 4));
      });

      isarTest('float change', () {
        isar.writeTxn((isar) {
          expect(isar.models.update(model.id, floatProp: 5), true);
        });
        expect(isar.models.get(model.id), model.copyWith(floatProp: 5));
      });

      isarTest('double change', () {
        isar.writeTxn((isar) {
          expect(isar.models.update(model.id, doubleProp: 6), true);
        });
        expect(isar.models.get(model.id), model.copyWith(doubleProp: 6));
      });

      isarTest('string change', () {
        isar.writeTxn((isar) {
          expect(isar.models.update(model.id, stringProp: 'world'), true);
        });
        expect(isar.models.get(model.id), model.copyWith(stringProp: 'world'));

        isar.writeTxn((isar) {
          expect(isar.models.update(model.id, stringProp: ''), true);
        });
        expect(isar.models.get(model.id), model.copyWith(stringProp: ''));

        isar.writeTxn((isar) {
          expect(isar.models.update(model.id, stringProp: 'loooong'), true);
        });
        expect(
          isar.models.get(model.id),
          model.copyWith(stringProp: 'loooong'),
        );
      });

      isarTest('nullable string change', () {
        isar.writeTxn((isar) {
          expect(isar.models.update(model.id, nullableStringProp: null), true);
        });
        expect(
          isar.models.get(model.id),
          model.copyWith(nullableStringProp: null),
        );

        isar.writeTxn((isar) {
          expect(
            isar.models.update(model.id, nullableStringProp: 'testaaaa'),
            true,
          );
        });
        expect(
          isar.models.get(model.id),
          model.copyWith(nullableStringProp: 'testaaaa'),
        );
      });

      isarTest('date change', () {
        isar.writeTxn((isar) {
          expect(
            isar.models.update(
              model.id,
              dateProp: DateTime.fromMillisecondsSinceEpoch(300, isUtc: true),
            ),
            true,
          );
        });

        expect(
          isar.models.get(model.id),
          model.copyWith(
            dateProp:
                DateTime.fromMillisecondsSinceEpoch(300, isUtc: true).toLocal(),
          ),
        );
      });
    });
  });
}
