from faker import Faker
from random import randint
import csv

fake = Faker('ru_RU')

def create_stops():
	rows = []
	with open('transport stops.csv', 'w', newline = '', encoding = 'utf8') as file:
		file.write('id,name,address,request_stop,install_year,electricity,rails\n')
		for i in range(900):
			a = fake.street_address()
			# id, name, address, request_stop, install_year, electricity, rails
			rows.append([
				i + 1,
				a,
				a,
				fake.boolean(),
				fake.date(),
				fake.boolean(),
				fake.boolean()
			])
		writer = csv.writer(file)
		writer.writerows(rows)

	return rows

def create_transport(stops_rows):
	rows = []
	with open('transport.csv', 'w', newline = '', encoding = 'utf8') as file:
		file.write('root_number,start_id,stop_id,transport_type,entry_date\n')
		for i in range(900):
			# root_number, start_id, stop_id, transport_type, entry_date
			trtype = fake.random_element(elements = ('автобус', 'маршрутка', 'электробус', 'трамвай', 'троллейбус'))

			start = fake.random_element(elements = stops_rows)
			while trtype == 'трамвай' and start[6] != True or trtype == 'троллейбус' and start[5] != True:
				start = fake.random_element(elements = stops_rows)

			stop = fake.random_element(elements = stops_rows)
			while trtype == 'трамвай' and stop[6] != True or trtype == 'троллейбус' and stop[5] != True:
				stop = fake.random_element(elements = stops_rows)

			rows.append([
				i + 1,
				start[0],
				stop[0],
				trtype,
				fake.date()
			])
		writer = csv.writer(file)
		writer.writerows(rows)

	return rows

def create_fare(transport_rows):
	rows = []
	with open('fare.csv', 'w', newline = '', encoding = 'utf8') as file:
		file.write('price,root_number,start_id,stop_id,day_time\n')
		for i in range(900):
			# price, root_number, start_id, stop_id, day_time
			rows.append([
				float(fake.numerify(text = '##.##')),
				transport_rows[i][0],
				transport_rows[i][1],
				transport_rows[i][2],
				fake.numerify(text = 'с # до #')
			])
		writer = csv.writer(file)
		writer.writerows(rows)

def create_timetable(transport_rows, stops_rows):
	rows = []
	with open('timetable.csv', 'w', newline = '', encoding = 'utf8') as file:
		file.write('timing,transport_stop_id,root,weekends,max_price\n')
		for i in range(900):
			stop = fake.random_element(elements = stops_rows)
			transport = fake.random_element(elements = transport_rows)
			while transport[3] == 'трамвай' and stop[6] != True or transport[3] == 'троллейбус' and stop[5] != True:
				stop = fake.random_element(elements = stops_rows)
				transport = fake.random_element(elements = transport_rows)

			# timing, transport_stop_id, root, weekends, max_price
			rows.append([
				fake.time(),
				stop[0],
				transport[0],
				fake.boolean(),
				float(fake.numerify(text = '##.##'))
			])
		writer = csv.writer(file)
		writer.writerows(rows)

def create_trst(transport_rows, stops_rows):
	rows = []
	with open('tr_trst.csv', 'w', newline = '', encoding = 'utf8') as file:
		file.write('root_number,transport_stop_id\n')
		for i in range(900):
			# root_number, transport_stop_id
			rows.append([
				transport_rows[i][0],
				stops_rows[i][0]
			])
		writer = csv.writer(file)
		writer.writerows(rows)


stops = create_stops()
transport = create_transport(stops)
create_fare(transport)
create_timetable(transport, stops)
create_trst(transport, stops)