#include <vector>
#include <algorithm>
#include <iostream>

using namespace std;

class Solution {
public:
	// create data structure
	struct DayTemperature {
		int day;
		int temperature;
	};

	// create sorting condition
	struct {
		bool operator()(DayTemperature lhs, DayTemperature rhs)
		{
			return lhs.temperature < rhs.temperature
			       || ((lhs.temperature == rhs.temperature)
			       && (lhs.day < rhs.day));
		}
	} DayTemperatureLess;

	// main function
	int findClosestDay(vector<DayTemperature>& dayTemperatures, int start, int end, int day)
	{
		int high = end;
		int low = start;
		int check = (high + low) / 2;
		int closestDay = -1;
		while(high >= low) {
			if(dayTemperatures[check].day > day) {
				high = check - 1;
				if((closestDay == -1) 
				   || ((dayTemperatures[check].day - day) < closestDay)) {
					closestDay = dayTemperatures[check].day - day;
				}
			} else if(dayTemperatures[check].day < day) {
				low = check + 1;
			} else {
				return dayTemperatures[check].day - day;
			}
			check = (high + low) / 2;
		}
		return closestDay;
	}

	vector<int> dailyTemperatures(vector<int>& temperatures) {
		// complexity = n*log(n) + n + n * log(n / x) * x
		// x = 1 -> complexity = n * log(n)
		// x = 71 -> complexity = n * log(n)

		// create DayTemperature vector
		vector<DayTemperature> dayTemperatures;
		for(int i = 0; i < temperatures.size(); i++) {
			dayTemperatures.push_back({i, temperatures[i]});
		}

		// sort
		sort(dayTemperatures.begin(), dayTemperatures.end(), DayTemperatureLess);

		// debug output
		cout << "sorted dayTemperatures:" << endl;
		for(auto dayTemperature: dayTemperatures) {
			cout << dayTemperature.temperature << ": " << dayTemperature.day << endl;
		}

		// create temperature changes vector
		vector<int> temperatureChanges;
		int previousTemperature = 0;
		for(int i = 0; i < dayTemperatures.size(); i++) {
			if(dayTemperatures[i].temperature > previousTemperature) {
				temperatureChanges.push_back(i);
				previousTemperature = dayTemperatures[i].temperature;
			}
		}
		temperatureChanges.push_back(dayTemperatures.size());

		// debug output
		cout << "temperatureChanges:" << endl;
		for(auto temperatureChange: temperatureChanges) {
			cout << temperatureChange << endl;
		}

		// do the main thing
		vector<int> returnValue(temperatures.size());
		previousTemperature = dayTemperatures[0].temperature;
		int temperatureChangesIdx = 1;
		for(auto dayTemperature: dayTemperatures) {
			if(dayTemperature.temperature > previousTemperature) {
				temperatureChangesIdx++;
				previousTemperature = dayTemperature.temperature;
			}
			if(temperatureChangesIdx == temperatureChanges.size() - 1) {
				returnValue[dayTemperature.day] = 0;
			} else {
				int closestDay = 0;
				bool start = true;
				for(int i = temperatureChangesIdx; i < temperatureChanges.size() - 1; i++) {
					int currentClosestDay = findClosestDay(dayTemperatures,
									       temperatureChanges[i],
									       temperatureChanges[i + 1] - 1,
									       dayTemperature.day + 1) + 1;
					if(currentClosestDay != 0 
					   && (currentClosestDay < closestDay || start)) {
						closestDay = currentClosestDay;
						start = false;
					}
				}
				returnValue[dayTemperature.day] = closestDay;
			}
		}

		// debug output
		cout << "returnValue:" << endl;
		for(auto value: returnValue) {
			cout << value << endl;
		}
		
		return returnValue;
	}
};

int main()
{
	vector<int> temperatures = {73,74,75,71,69,72,76,73};
	Solution sol;
	sol.dailyTemperatures(temperatures);
	return 0;
}
