const gpaBtn = document.getElementById("GPA")
const cgpaBtn = document.getElementById("CGPA")
let max = 0
gpaBtn.addEventListener('click', () =>{
	type('GPA:', gpaBtn, cgpaBtn)
	max = 12
})
cgpaBtn.addEventListener('click', () =>{
	type('CGPA:', cgpaBtn, gpaBtn)
	max= 30
})

let Scores = [];
const courseNum = document.getElementById("col");
courseNum.addEventListener('input', () => {
	if (courseNum.valueAsNumber > max || courseNum.valueAsNumber !== parseInt(courseNum.valueAsNumber)){
		courseNum.value = ''
		courseNum.classList.add("error_cell")
		courseNum.placeholder = 'Select GPA or CGPA'
	} else{
		courseNum.placeholder = 'Enter Number Of Courses'
		courseNum.classList.remove('error_cell')
		return
	}
})
function type(n, m, y){
	document.getElementById("display1").innerHTML = n
	m.classList.add("select")
	y.classList.remove("select")
}

let Grades = [];
let Credithour = []
let Courses = [];
let GP  = {'A': 5, 'B': 4, 'C': 3, 'D': 2, 'E': 1, 'F':0}
Gradepoint = []
function gentable() {
    let cols = 4;
    const rows = document.getElementById("col").valueAsNumber;

    // Create the table HTML
    let tableHTML = "<table border='1' id='myt'>";

    // Create the table header
    let heading = ['Courses', 'Score', 'Grade', 'Credit Hour'];
    let datatype = ['text', 'number', 'text', 'number'];
    let len = ['6', '3', '1', '1']
    let placeh = ['Enter Course', 'Optional', 'Enter grade', 'Enter credit hour'];
    tableHTML += "<thead><tr>";
    for (let p = 0; p < cols; p++) {
        tableHTML += `<th>${heading[p]}</th>`;
    }
    tableHTML += "</tr></thead>";

    // Create the table body
    tableHTML += "<tbody>";
    for (let i = 0; i < rows; i++) {
        tableHTML += "<tr>";
        for (let j = 0; j < cols; j++) {
            tableHTML += `<td><input type='${datatype[j]}' name='row${i}_col${j}' id='input_${i}_${j}' placeholder='${placeh[j]}' maxlength = ${len[j]} ></td>`;
        }
        tableHTML += "</tr>";
    }
    tableHTML += "</tbody>";

    // Close the table
    tableHTML += "</table>";

    // Display the table in the HTML element with id "table-container"
    document.getElementById("table-container").innerHTML = tableHTML;
    getdata()
    modattributes(rows)
    errorcheck(rows,cols)
  
    
}

const btn = document.getElementById("submit");
btn.addEventListener('keydown', (event) => {
    if (event.key === 'Enter') {
        gentable();
    }
});

btn.addEventListener('click', () => {
	if (courseNum.value === '' || courseNum.valueAsNumber ===0  ){
		alert("Please Enter The Number Of Courses")
	} else{
    gentable();
	}
});
function getdata(){
  const rows = document.getElementById("col").valueAsNumber;
	for (let i = 0; i < rows; i++) {
		    let course= document.getElementById(`input_${i}_0`);
        course.addEventListener('input', () => {
            // Get the current value of the input field
            let courseinput = course.value.toUpperCase();
            // Update the Courses array with the new value
            course.value = courseinput
            Courses[i] = courseinput
        })
            
        let score = document.getElementById(`input_${i}_1`);
        score.addEventListener('input', () => {
        	  const max = 100
            // Get the current value of the input field
            if (score.valueAsNumber > max || score.valueAsNumber !== parseInt(score.valueAsNumber)){
            	score.value = ''
            	Scores[i] = score.valueAsNumber;
            	for(let i = 0 ; i < rows; i++){
            		document.getElementById(`input_${i}_1`).placeholder = 'optional'
            	}
            	
            } else{
            	for(let i = 0 ; i < rows; i++){
            		document.getElementById(`input_${i}_1`).placeholder = 'Enter Score'
            	}
            	Scores[i] = score.valueAsNumber;
            	const gradesys = new GradingSystem()
            	let sysgrade = gradesys.getGrade(score.valueAsNumber)
            	document.getElementById(`input_${i}_2`).value = sysgrade
            	Grades[i] = sysgrade
            	Gradepoint[i] = GP[sysgrade]
            }
        });
        
        
        //for grades
        let grade= document.getElementById(`input_${i}_2`);
        grade.addEventListener('input', () => {
            let gradeinput = grade.value.toUpperCase();
            if(GP[gradeinput]!== undefined){
            	grade.value = gradeinput
            	Gradepoint[i] = GP[gradeinput];
            	document.getElementById(`input_${i}_1`).value = ''
            	document.getElementById(`input_${i}_1`).placeholder = 'optional'
            }
            	else{
            		grade.value = ''
            		Gradepoint[i] = 0
            	}
            calculator(rows)
        });
        //for credit hour
        let credhr = document.getElementById(`input_${i}_3`);
        credhr.addEventListener('input', () => {
        	  const max = 5
            if (credhr.valueAsNumber > max || credhr.valueAsNumber !==parseInt(credhr.valueAsNumber)){
            	credhr.value = ''
            	Credithour[i] = 0
            } else{
            	Credithour[i] = credhr.valueAsNumber
            }
            calculator(rows)
            
        });
  }
}
function modattributes(n){
	for(let i = 0; i < n; i++){
		let inputGfield = document.getElementById(`input_${i}_2`);
		inputGfield.setAttribute('list', 'grades')
		let inputCfield = document.getElementById(`input_${i}_3`);
		inputCfield.setAttribute('list', 'creditun')
	}
}
function errorcheck(row, col) {
    for (let i = 0; i < row; i++) {
        let scorecell = document.getElementById(`input_${i}_1`); // Reference to the score cell

        for (let j = 0; j < col; j++) {
            let cell = document.getElementById(`input_${i}_${j}`);
            let origplace = cell.placeholder; // Store the original placeholder

            cell.addEventListener('blur', () => {
                // Check if the cell is empty
                if (cell.value.trim() === "") {
                    // Skip adding error for the score cell if it's empty
                    if (cell === scorecell) {
                        return;
                    } else {
                        // Add error class and update placeholder text
                        cell.classList.add("error_cell");
                        cell.placeholder = "Enter Value";
                    }
                } else {
                    // Remove the error class and restore the original placeholder text
                    cell.classList.remove("error_cell");
                    cell.placeholder = origplace;
                }
            });
        }
    }
}

class GradingSystem {
    // Method to determine the grade based on the score
    getGrade(score) {
    	let grade =''
        if (score >= 70 && score <= 100) {
        	return 'A'
        } else if (score >= 60 && score < 70) {
           return 'B';
        } else if (score >= 50 && score < 60) {
            return 'C';
        } else if (score >= 45 && score < 50) {
            return 'D';
        } else if (score >= 40 && score < 45) {
            return 'E';
        } else if (score >= 0 && score < 40) {
            return 'F';
        } else {
            return;
        }
        
        
    }
}


function calculator(row){
	let sum = 0
	let credHrSum = 0
	for (let i = 0; i < row; i++){
		const product = Gradepoint[i] * Credithour[i]
		sum += product
		credHrSum += Credithour[i]
	}
	const GPA = credHrSum > 0 ? sum / credHrSum : 0;
	document.getElementById('display2').innerHTML = GPA.toFixed(2)
}
function darkmode(){
	document.querySelector('html').classList.toggle('dark-mode')
}
function refreshBtn(){
	calculator(document.getElementById('col').valueAsNumber)
}



